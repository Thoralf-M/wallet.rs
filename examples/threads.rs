// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example threads --release

// In this example we will spam transactions from multiple threads simultaneously to our own address

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_block::{
        output::{
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            AliasId, BasicOutputBuilder, NftId, Output, OutputId,
        },
        payload::transaction::{TransactionEssence, TransactionPayload},
    },
    constants::SHIMMER_COIN_TYPE,
};
use iota_wallet::{
    account::{SyncOptions, TransactionOptions},
    account_manager::AccountManager,
    logger::init_logger,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    ClientOptions, NativeTokenOptions, NftOptions, Result,
};
use log::LevelFilter;
use primitive_types::U256;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger("threads.log", LevelFilter::Debug)?;
    let client_options = ClientOptions::new()
        .with_node("http:localhost:14265")?
        .with_node_sync_disabled();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let mnemonic = env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC").unwrap();
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(&mnemonic)?;

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
    let address = account.list_addresses().await?[0].address().clone();
    println!("{}", address.to_bech32());

    let balance = account.sync(None).await?;
    println!("Balance: {:?}", balance);

    if balance.base_coin.available == 0 {
        panic!("Account has no available balance");
    }

    for _ in 0..5 {
        let mut threads = Vec::new();
        for n in 0..1 {
            let account_ = account.clone();
            let address_ = *address.as_ref();

            threads.push(async move {
                tokio::spawn(async move {
                    // send transaction
                    // let outputs = vec![
                    //     BasicOutputBuilder::new_with_amount(1_000_000)?
                    //         .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address_)))
                    //         .finish_output()?;
                    //     // amount of outputs in the transaction (one additional output might be added for the remaining amount)
                    //     1
                    // ];
                    // // Skip sync here, we already synced before and don't need to do it again for every transaction
                    // let tx = account_
                    //     .send(
                    //         outputs,
                    //         Some(TransactionOptions {
                    //             skip_sync: true,
                    //             ..Default::default()
                    //         }),
                    //     )
                    //     .await?;
                    // if let Some(block_id) = tx.block_id {
                    //     println!(
                    //         "Block from thread {} sent: http://localhost:14265/api/v2/blocks/{}",
                    //         n, block_id
                    //     );
                    // }

                    // let nft_options = vec![NftOptions {
                    //     address: None,
                    //     immutable_metadata: Some(b"some immutable nft metadata".to_vec()),
                    //     metadata: Some(b"some nft metadata".to_vec()),
                    // }];

                    // let transaction = account_.mint_nfts(nft_options, None).await?;
                    // println!(
                    //     "Mint nft ransaction: {} Block sent: http://localhost:14265/api/v2/blocks/{}",
                    //     transaction.transaction_id,
                    //     transaction.block_id.expect("No block created yet")
                    // );

                    // let output_id = get_nft_output_id(&transaction.payload);
                    // tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                    // let _ = match transaction.block_id {
                        // Some(block_id) => account_.retry_until_included(&block_id, None, None).await?,
                        // None => {
                            // return Err(iota_wallet::Error::BurningOrMeltingFailed(
                                // "Melt native token transaction failed to submitted".to_string(),
                            // ));
                        // }
                    // };
                    account_
                        .sync(Some(SyncOptions {
                            force_syncing: true,
                            ..Default::default()
                        }))
                        .await?;
                    // let transaction = account_.burn_nft(NftId::from(output_id), None).await?;

                    // println!(
                    //     "Burn nft Transaction: {} Block sent: http://localhost:14265/api/v2/blocks/{}",
                    //     transaction.transaction_id,
                    //     transaction.block_id.expect("No block created yet")
                    // );

                    let native_token_options = NativeTokenOptions {
                        account_address: None,
                        circulating_supply: U256::from(100),
                        maximum_supply: U256::from(100),
                        foundry_metadata: None,
                    };

                    let transaction = account_.mint_native_token(native_token_options, None).await?;
                    println!(
                        "mint native token Transaction: {} Block sent: http://localhost:14265/api/v2/blocks/{}",
                        transaction.transaction.transaction_id,
                        transaction.transaction.block_id.expect("No block created yet")
                    );

                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                    let _ = match transaction.transaction.block_id {
                        Some(block_id) => account_.retry_until_included(&block_id, None, None).await?,
                        None => {
                            return Err(iota_wallet::Error::BurningOrMeltingFailed(
                                "Melt native token transaction failed to submitted".to_string(),
                            ));
                        }
                    };
                    account_
                        .sync(Some(SyncOptions {
                            force_syncing: true,
                            ..Default::default()
                        }))
                        .await?;


                    // Melt some of the circulating supply
                    // let melt_amount = U256::from(100);
                    // let transaction = account_
                    //     .melt_native_token((transaction.token_id, melt_amount), None)
                    //     .await?;

                    // tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                    // let _ = match transaction.block_id {
                    //     Some(block_id) => account_.retry_until_included(&block_id, None, None).await?,
                    //     None => {
                    //         return Err(iota_wallet::Error::BurningOrMeltingFailed(
                    //             "Melt native token transaction failed to submitted".to_string(),
                    //         ));
                    //     }
                    // };

                    // account_
                    //     .sync(Some(SyncOptions {
                    //         force_syncing: true,
                    //         ..Default::default()
                    //     }))
                    //     .await?;
                    // let alias_id = get_alias_id(&transaction.payload);
                    // let transaction = account_.destroy_alias(alias_id, None).await?;
                    // println!(
                    //     "Destroy alias Transaction: {} Block sent: http://localhost:14265/api/v2/blocks/{}",
                    //     transaction.transaction_id,
                    //     transaction.block_id.expect("No block created yet")
                    // );

                    println!("reached end");
                    iota_wallet::Result::Ok(n)
                })
                .await
            });
        }

        let results = futures::future::try_join_all(threads).await?;
        for thread in results {
            if let Err(e) = thread {
                println!("{e}");
                // Sync when getting an error, because that's probably when no outputs are available anymore
                println!("Syncing account...");
                account.sync(None).await?;
            }
        }
    }
    Ok(())
}

// helper function to get the output id for the first NFT output
fn get_nft_output_id(tx_payload: &TransactionPayload) -> OutputId {
    let TransactionEssence::Regular(regular) = tx_payload.essence();
    for (index, output) in regular.outputs().iter().enumerate() {
        if let Output::Nft(_nft_output) = output {
            return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
        }
    }
    panic!("No nft output in transaction essence")
}

// helper function to get the alias id for the first alias output
fn get_alias_id(tx_payload: &TransactionPayload) -> AliasId {
    let TransactionEssence::Regular(regular) = tx_payload.essence();
    for (_index, output) in regular.outputs().iter().enumerate() {
        if let Output::Alias(alias_output) = output {
            return *alias_output.alias_id();
        }
    }
    panic!("No alias output in transaction essence")
}
