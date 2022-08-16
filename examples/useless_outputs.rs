// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example useless_outputs --release

// In this example we will spam useless outputs to all addresses

use std::{
    collections::HashSet,
    env,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use dotenv::dotenv;
use iota_client::{
    block::{
        address::{Address, AliasAddress, NftAddress},
        output::{
            dto::OutputDto,
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition, UnlockCondition,
            },
            AliasId, BasicOutputBuilder, NftId, Output, OutputId,
        },
        payload::transaction::TransactionId,
    },
    constants::SHIMMER_COIN_TYPE,
    request_funds_from_faucet,
};
use iota_wallet::{
    account_manager::AccountManager,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    ClientOptions, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let client_options = ClientOptions::new()
        .with_node(&env::var("NODE_URL").unwrap())?
        .with_pow_worker_count(5)
        .with_node_sync_disabled();

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC").unwrap())?;

    let manager = AccountManager::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .finish()
        .await?;

    // Get account or create a new one
    let account_alias = "thread_account";
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

    // One address gets generated during account creation
    let account_address = account.list_addresses().await?[0].address().clone();
    println!("{}", account_address.to_bech32());

    let balance = account.sync(None).await?;
    println!("Balance: {:?}", balance);

    if balance.base_coin.available == 0 {
        panic!("Account has no available balance");
    }

    // Get addresses from basic outputs
    let mut all_addresses = HashSet::new();
    let basic_output_ids = account.client().basic_output_ids(vec![]).await?;
    println!("Basic output ids : {}", basic_output_ids.len());

    let basic_outputs = account.client().get_outputs(basic_output_ids).await?;
    for basic_output_response in basic_outputs {
        if let OutputDto::Basic(basic_output) = basic_output_response.output {
            for unlock_condition in basic_output.unlock_conditions {
                if let Ok(UnlockCondition::Address(address)) = UnlockCondition::try_from(&unlock_condition) {
                    all_addresses.insert(*address.address());
                    continue;
                }
            }
        }
    }

    let nft_output_ids = account.client().nft_output_ids(vec![]).await?;
    println!("Nft output ids : {}", nft_output_ids.len());
    let nft_outputs = account.client().get_outputs(nft_output_ids).await?;
    for nft_output_response in nft_outputs {
        if let OutputDto::Nft(nft_output) = nft_output_response.output {
            let transaction_id = TransactionId::from_str(&nft_output_response.metadata.transaction_id)?;
            let nft_id = NftId::try_from(&nft_output.nft_id)?.or_from_output_id(OutputId::new(
                transaction_id,
                nft_output_response.metadata.output_index,
            )?);
            all_addresses.insert(Address::Nft(NftAddress::new(nft_id)));
            for unlock_condition in nft_output.unlock_conditions {
                if let Ok(UnlockCondition::Address(address)) = UnlockCondition::try_from(&unlock_condition) {
                    all_addresses.insert(*address.address());
                    continue;
                }
            }
        }
    }

    let alias_output_ids = account.client().alias_output_ids(vec![]).await?;
    println!("Alias output ids : {}", alias_output_ids.len());
    let alias_outputs = account.client().get_outputs(alias_output_ids).await?;
    for alias_output_response in alias_outputs {
        if let OutputDto::Alias(alias_output) = alias_output_response.output {
            let transaction_id = TransactionId::from_str(&alias_output_response.metadata.transaction_id)?;
            let alias_id = AliasId::try_from(&alias_output.alias_id)?.or_from_output_id(OutputId::new(
                transaction_id,
                alias_output_response.metadata.output_index,
            )?);
            all_addresses.insert(Address::Alias(AliasAddress::new(alias_id)));
            for unlock_condition in alias_output.unlock_conditions {
                if let Ok(UnlockCondition::GovernorAddress(address)) = UnlockCondition::try_from(&unlock_condition) {
                    all_addresses.insert(*address.address());
                    continue;
                }
            }
        }
    }

    // foundries aren't needed, since we have all alias outputs already

    println!("addresses: {}", all_addresses.len());

    for (index, receiver_addresses) in all_addresses
        .into_iter()
        .collect::<Vec<Address>>()
        .chunks(127)
        .enumerate()
    {
        let one_week = (SystemTime::now() + Duration::from_secs(7 * 24 * 3600))
            .duration_since(UNIX_EPOCH)
            .expect("clock went backwards")
            .as_secs()
            .try_into()
            .unwrap();

        // send transaction
        let outputs = receiver_addresses
            .into_iter()
            .map(|receiver_address| {
                BasicOutputBuilder::new_with_amount(50600)?
                    .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(*receiver_address)))
                    .add_unlock_condition(UnlockCondition::StorageDepositReturn(
                        StorageDepositReturnUnlockCondition::new(*account_address.as_ref(), 50600)?,
                    ))
                    .add_unlock_condition(UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                        *account_address.as_ref(),
                        one_week,
                    )?))
                    .finish_output()
                    .map_err(From::from)
            })
            .collect::<Result<Vec<Output>>>()?;

        match account.send(outputs, None).await {
            Ok(tx) => {
                if let Some(block_id) = tx.block_id {
                    println!(
                        "{index} Block sent: {}/api/core/v2/blocks/{}",
                        &env::var("NODE_URL").unwrap(),
                        block_id
                    );
                }
            }
            Err(e) => {
                println!("{e}");

                let faucet_response =
                    request_funds_from_faucet(&env::var("FAUCET_URL").unwrap(), &account_address.to_bech32()).await?;
                println!("{}", faucet_response);

                tokio::time::sleep(Duration::from_secs(10)).await;

                // Sync when getting an error, because that's probably when no outputs are available anymore
                println!("Syncing account...");
                account.sync(None).await?;
            }
        }
    }

    Ok(())
}
