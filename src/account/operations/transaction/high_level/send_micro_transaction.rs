// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    api::PreparedTransactionData,
    bee_block::{
        address::Address,
        output::{
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition, UnlockCondition,
            },
            BasicOutputBuilder,
        },
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    account::{
        constants::DEFAULT_EXPIRATION_TIME,
        handle::AccountHandle,
        operations::transaction::{
            high_level::minimum_storage_deposit::minimum_storage_deposit_basic_native_tokens, Transaction,
        },
        TransactionOptions,
    },
    Error,
};

/// address with amount for `send_micro_transaction()`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressWithMicroAmount {
    /// Bech32 encoded address
    pub address: String,
    /// Amount below the minimum storage deposit
    pub amount: u64,
    /// Bech32 encoded address return address, to which the storage deposit will be returned. Default will use the
    /// first address of the account
    pub return_address: Option<String>,
    /// Expiration in seconds, after which the output will be available for the sender again, if not spent by the
    /// receiver before. Default is 1 day
    pub expiration: Option<u32>,
}

impl AccountHandle {
    /// Function to send micro transactions by using the [StorageDepositReturnUnlockCondition] with an
    /// [ExpirationUnlockCondition]. Will call [AccountHandle.send()](crate::account::handle::AccountHandle.send),
    /// the options can define the RemainderValueStrategy or custom inputs.
    /// Address needs to be Bech32 encoded
    /// ```ignore
    /// let outputs = vec![AddressWithMicroAmount{
    ///    address: "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu".to_string(),
    ///    amount: 1,
    ///    return_address: None,
    ///    expiration: None,
    /// }];
    ///
    /// let transaction = account_handle.send_micro_transaction(outputs, None ).await?;
    ///
    /// println!(
    ///    "Transaction: {} Block sent: http://localhost:14265/api/v2/blocks/{}",
    ///    transaction.transaction_id,
    ///    transaction.block_id.expect("No block created yet")
    /// );
    /// ```
    pub async fn send_micro_transaction(
        &self,
        addresses_with_micro_amount: Vec<AddressWithMicroAmount>,
        options: Option<TransactionOptions>,
    ) -> crate::Result<Transaction> {
        log::debug!("[TRANSACTION] send_micro_transaction");
        let prepared_trasacton = self
            .prepare_send_micro_transaction(addresses_with_micro_amount, options)
            .await?;
        self.sign_and_submit_transaction(prepared_trasacton).await
    }

    /// Function to prepare the transaction for
    /// [AccountHandle.send_micro_transaction()](crate::account::handle::AccountHandle.send_micro_transaction)
    async fn prepare_send_micro_transaction(
        &self,
        addresses_with_micro_amount: Vec<AddressWithMicroAmount>,
        options: Option<TransactionOptions>,
    ) -> crate::Result<PreparedTransactionData> {
        let byte_cost_config = self.client.get_byte_cost_config().await?;

        let account_addresses = self.list_addresses().await?;
        let return_address = account_addresses.first().ok_or(Error::FailedToGetRemainder)?;

        let local_time = self.client.get_time_checked().await?;

        let mut outputs = Vec::new();
        for address_with_amount in addresses_with_micro_amount {
            let (_bech32_hrp, address) = Address::try_from_bech32(&address_with_amount.address)?;
            // get minimum required amount for such an output, so we don't lock more than required
            // We have to check it for every output individually, because different address types and amount of
            // different native tokens require a differen storage deposit
            let storage_deposit_amount = minimum_storage_deposit_basic_native_tokens(
                &byte_cost_config,
                &address,
                &return_address.address.inner,
                None,
            )?;

            let expiration_time = match address_with_amount.expiration {
                Some(expiration_time) => local_time + expiration_time,
                None => local_time + DEFAULT_EXPIRATION_TIME,
            };

            outputs.push(
                // Add address_and_amount.amount+storage_deposit_amount, so receiver can get address_and_amount.amount
                BasicOutputBuilder::new_with_amount(address_with_amount.amount + storage_deposit_amount)?
                    .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
                    .add_unlock_condition(UnlockCondition::StorageDepositReturn(
                        // We send the storage_deposit_amount back to the sender, so only the additional amount is sent
                        StorageDepositReturnUnlockCondition::new(return_address.address.inner, storage_deposit_amount)?,
                    ))
                    .add_unlock_condition(UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                        address,
                        expiration_time,
                    )?))
                    .finish_output()?,
            )
        }

        self.prepare_transaction(outputs, options).await
    }
}
