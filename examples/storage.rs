// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example storage --release

use std::time::Instant;

use iota_wallet::{
    account_manager::AccountManager,
    iota_client::constants::SHIMMER_COIN_TYPE,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    ClientOptions, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(
        "flame fever pig forward exact dash body idea link scrub tennis minute surge unaware prosper over waste kitten ceiling human knife arch situate civil",
    )?;

    let client_options = ClientOptions::new()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled();

    let manager = AccountManager::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_storage_path("wallet-database")
        .finish()
        .await?;

    // Get account or create a new one
    let account_alias = "logger";
    let account = match manager.get_account(account_alias.to_string()).await {
        Ok(account) => account,
        _ => {
            // first we'll create an example account and store it
            manager
                .create_account()
                .with_alias(account_alias.to_string())
                .finish()
                .await?
        }
    };

    let addresses = account.generate_addresses(3, None).await?;
    let mut bech32_addresses = Vec::new();
    for address in addresses {
        bech32_addresses.push(address.address().to_bech32());
    }
    println!("Generated new addresses: {:#?}", bech32_addresses);

    println!("addresses: {:?}", account.list_addresses().await?.len());
    let now = Instant::now();
    let balance = account.sync(None).await?;
    println!("Syncing took: {:.2?}", now.elapsed());
    println!("Balance: {:?}", balance);

    #[cfg(debug_assertions)]
    manager.verify_integrity().await?;
    Ok(())
}
