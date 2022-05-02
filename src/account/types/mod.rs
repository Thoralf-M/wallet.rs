// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Address types used in the account
pub(crate) mod address;
pub use address::{AccountAddress, AddressWithUnspentOutputs};
/// Custom de/serialization for [`address::AddressWrapper`]
pub(crate) mod address_serde;

use std::{collections::HashMap, str::FromStr};

use crypto::keys::slip10::Chain;
use iota_client::{
    bee_message::{
        address::Address,
        output::{AliasId, FoundryId, NftId, Output, OutputId, TokenId},
        payload::transaction::TransactionPayload,
        MessageId,
    },
    bee_rest_api::types::responses::OutputResponse,
    secret::types::InputSigningData,
};
use primitive_types::U256;
use serde::{Deserialize, Deserializer, Serialize};

/// The balance of an account, returned from [`crate::account::handle::AccountHandle::sync()`] and
/// [`crate::account::handle::AccountHandle::balance()`].
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    // Total amount
    pub total: u64,
    // Balance that can currently be spend
    pub available: u64,
    // Current required storage deposit amount
    #[serde(rename = "requiredStorageDeposit")]
    pub required_storage_deposit: u64,
    // Native tokens
    #[serde(rename = "nativeTokens")]
    pub native_tokens: HashMap<TokenId, U256>,
    // Nfts
    pub nfts: Vec<NftId>,
    // Aliases
    pub aliases: Vec<AliasId>,
    // Foundries
    pub foundries: Vec<FoundryId>,
    // Outputs with multiple unlock conditions and if they can currently be spent or not. If there is a
    // [`TimelockUnlockCondition`] or [`ExpirationUnlockCondition`] this can change at any time
    #[serde(rename = "potentiallyLockedOutputs")]
    pub potentially_locked_outputs: HashMap<OutputId, bool>,
}

/// An output with metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputData {
    /// The output id
    #[serde(rename = "outputId")]
    pub output_id: OutputId,
    // todo: remove OutputResponse and store metadata alone
    /// The output response
    #[serde(rename = "outputResponse")]
    pub output_response: OutputResponse,
    /// The actual Output
    pub output: Output,
    // The output amount
    pub amount: u64,
    /// If an output is spent
    #[serde(rename = "isSpent")]
    pub is_spent: bool,
    /// Associated account address.
    pub address: Address,
    /// Network ID
    #[serde(rename = "networkId")]
    pub network_id: u64,
    pub remainder: bool,
    // bip32 path
    pub chain: Option<Chain>,
}

impl OutputData {
    pub fn input_signing_data(&self) -> crate::Result<InputSigningData> {
        Ok(InputSigningData {
            output_response: self.output_response.clone(),
            chain: self.chain.clone(),
            bech32_address: self.address.to_bech32("atoi"),
        })
    }
}

/// A transaction with metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub payload: TransactionPayload,
    pub message_id: Option<MessageId>,
    pub inclusion_state: InclusionState,
    // remove because we have a timestamp in the outputs?
    pub timestamp: u128,
    // network id to ignore outputs when set_client_options is used to switch to another network
    pub network_id: u64,
    // set if the transaction was created by the wallet or if it was sent by someone else and is incoming
    pub incoming: bool,
}

/// Possible InclusionStates for transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InclusionState {
    Pending,
    Confirmed,
    Conflicting,
}

/// The output kind enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputKind {
    /// Alias output.
    Alias,
    /// Basic output.
    Basic,
    /// Foundry output.
    Foundry,
    /// Nft output.
    Nft,
    /// Treasury output.
    Treasury,
}

impl FromStr for OutputKind {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "Alias" => Self::Alias,
            "Basic" => Self::Basic,
            "Foundry" => Self::Foundry,
            "Nft" => Self::Nft,
            "Treasury" => Self::Treasury,
            _ => return Err(crate::Error::InvalidOutputKind(s.to_string())),
        };
        Ok(kind)
    }
}

/// The account identifier.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AccountIdentifier {
    /// Account alias as identifier.
    Alias(String),
    /// An index identifier.
    Index(u32),
}

// Custom deserialize because the index could also be encoded as String
impl<'de> Deserialize<'de> for AccountIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;
        let v = Value::deserialize(deserializer)?;
        Ok(match v.as_u64() {
            Some(number) => {
                let index: u32 =
                    u32::try_from(number).map_err(|_| D::Error::custom("Account index is greater than u32::MAX"))?;
                AccountIdentifier::Index(index)
            }
            None => {
                let alias_or_index_str = v
                    .as_str()
                    .ok_or_else(|| D::Error::custom("AccountIdentifier is no number or string"))?;
                AccountIdentifier::from(alias_or_index_str)
            }
        })
    }
}

// When the identifier is a string.
impl From<&str> for AccountIdentifier {
    fn from(value: &str) -> Self {
        match u32::from_str(value) {
            Ok(index) => Self::Index(index),
            Err(_) => Self::Alias(value.to_string()),
        }
    }
}

impl From<String> for AccountIdentifier {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&String> for AccountIdentifier {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

// When the identifier is an index.
impl From<u32> for AccountIdentifier {
    fn from(value: u32) -> Self {
        Self::Index(value)
    }
}

/// BIP 44 coin type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CoinType {
    IOTA = 4218,
    Shimmer = 4219,
}

impl Default for CoinType {
    fn default() -> Self {
        CoinType::Shimmer
    }
}

impl TryFrom<u32> for CoinType {
    type Error = crate::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let coin_type = match value {
            x if x == CoinType::IOTA as u32 => CoinType::IOTA,
            x if x == CoinType::Shimmer as u32 => CoinType::Shimmer,
            _ => return Err(crate::Error::InvalidCoinType(value)),
        };
        Ok(coin_type)
    }
}