// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{ops::Deref, sync::Arc};

use iota_client::{
    bee_block::{
        output::{FoundryId, Output, OutputId, TokenId},
        payload::transaction::{TransactionId, TransactionPayload},
    },
    bee_rest_api::types::responses::OutputResponse,
    secret::SecretManager,
    Client,
};
use tokio::sync::{Mutex, RwLock};

#[cfg(feature = "events")]
use crate::events::EventEmitter;
#[cfg(feature = "storage")]
use crate::storage::manager::StorageManagerHandle;
use crate::{
    account::{
        types::{
            address::{AccountAddress, AddressWithUnspentOutputs},
            OutputData, Transaction,
        },
        Account,
    },
    Result,
};

/// A thread guard over an account, so we can lock the account during operations.
#[derive(Debug, Clone)]
pub struct AccountHandle {
    account: Arc<RwLock<Account>>,
    pub(crate) client: Client,
    pub(crate) secret_manager: Arc<RwLock<SecretManager>>,
    // mutex to prevent multiple sync calls at the same or almost the same time, the u128 is a timestamp
    // if the last synced time was < `MIN_SYNC_INTERVAL` second ago, we don't sync, but only calculate the balance
    // again, because sending transactions can change that
    pub(crate) last_synced: Arc<Mutex<u128>>,
    #[cfg(feature = "events")]
    pub(crate) event_emitter: Arc<Mutex<EventEmitter>>,
    #[cfg(feature = "storage")]
    pub(crate) storage_manager: StorageManagerHandle,
}

impl AccountHandle {
    /// Create a new AccountHandle with an Account
    pub(crate) fn new(
        account: Account,
        client: Client,
        secret_manager: Arc<RwLock<SecretManager>>,
        #[cfg(feature = "events")] event_emitter: Arc<Mutex<EventEmitter>>,
        #[cfg(feature = "storage")] storage_manager: StorageManagerHandle,
    ) -> Self {
        Self {
            account: Arc::new(RwLock::new(account)),
            client,
            secret_manager,
            last_synced: Default::default(),
            #[cfg(feature = "events")]
            event_emitter,
            #[cfg(feature = "storage")]
            storage_manager,
        }
    }

    pub async fn alias(&self) -> String {
        self.read().await.alias.clone()
    }

    /// Get the [`OutputData`] of an output stored in the account
    pub async fn get_output(&self, output_id: &OutputId) -> Option<OutputData> {
        let account = self.read().await;
        account.outputs().get(output_id).cloned()
    }

    /// Get the [`Output`] that minted a native token by the token ID. First try to get it
    /// from the account, if it isn't in the account try to get it from the node
    pub async fn get_foundry_output(&self, native_token_id: TokenId) -> Result<Output> {
        let account = self.read().await;
        let foundry_id = FoundryId::from(native_token_id);
        for output_data in account.outputs().values() {
            if let Output::Foundry(foundry_output) = &output_data.output {
                if foundry_output.id() == foundry_id {
                    return Ok(output_data.output.clone());
                }
            }
        }
        drop(account);

        // Foundry was not found in the account, try to get it from the node
        let foundry_output_id = self.client.foundry_output_id(foundry_id).await?;
        let output_response = self.client.get_output(&foundry_output_id).await?;

        Ok(Output::try_from(&output_response.output)?)
    }

    /// Get the [`Transaction`] of a transaction stored in the account
    pub async fn get_transaction(&self, transaction_id: &TransactionId) -> Option<Transaction> {
        let account = self.read().await;
        account.transactions().get(transaction_id).cloned()
    }

    /// Get the transaction with inputs of an incoming transaction stored in the account
    /// List might not be complete, if the node pruned the data already
    pub async fn get_incoming_transaction_data(
        &self,
        transaction_id: &TransactionId,
    ) -> Option<(TransactionPayload, Vec<OutputResponse>)> {
        let account = self.read().await;
        account.incoming_transactions().get(transaction_id).cloned()
    }

    /// Returns all addresses of the account
    pub async fn list_addresses(&self) -> Result<Vec<AccountAddress>> {
        let account = self.read().await;
        let mut all_addresses = account.public_addresses().clone();
        all_addresses.extend(account.internal_addresses().clone());
        Ok(all_addresses.to_vec())
    }

    /// Returns only addresses of the account with balance
    pub async fn list_addresses_with_unspent_outputs(&self) -> Result<Vec<AddressWithUnspentOutputs>> {
        let account = self.read().await;
        Ok(account.addresses_with_unspent_outputs().to_vec())
    }

    /// Returns all outputs of the account
    pub async fn list_outputs(&self) -> Result<Vec<OutputData>> {
        let account = self.read().await;
        let mut outputs = Vec::new();
        for output in account.outputs.values() {
            outputs.push(output.clone());
        }
        Ok(outputs)
    }

    /// Returns all unspent outputs of the account
    pub async fn list_unspent_outputs(&self) -> Result<Vec<OutputData>> {
        let account = self.read().await;
        let mut outputs = Vec::new();
        for output in account.unspent_outputs.values() {
            outputs.push(output.clone());
        }
        Ok(outputs)
    }

    /// Returns all transaction of the account
    pub async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        let account = self.read().await;
        let mut transactions = Vec::new();
        for transaction in account.transactions.values() {
            transactions.push(transaction.clone());
        }
        Ok(transactions)
    }

    /// Returns all pending transaction of the account
    pub async fn list_pending_transactions(&self) -> Result<Vec<Transaction>> {
        let account = self.read().await;
        let mut transactions = Vec::new();
        for transaction_id in &account.pending_transactions {
            if let Some(transaction) = account.transactions.get(transaction_id) {
                transactions.push(transaction.clone());
            }
        }
        Ok(transactions)
    }

    #[cfg(feature = "storage")]
    /// Save the account to the database, accepts the updated_account as option so we don't need to drop it before
    /// saving
    pub(crate) async fn save(&self, updated_account: Option<&Account>) -> Result<()> {
        log::debug!("[save] saving account to database");
        match updated_account {
            Some(account) => self.storage_manager.lock().await.save_account(account).await,
            None => {
                let account = self.read().await;
                self.storage_manager.lock().await.save_account(&account).await
            }
        }
    }
}

// impl Deref so we can use `account_handle.read()` instead of `account_handle.account.read()`
impl Deref for AccountHandle {
    type Target = RwLock<Account>;
    fn deref(&self) -> &Self::Target {
        self.account.deref()
    }
}
