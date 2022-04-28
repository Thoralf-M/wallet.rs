// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{path::PathBuf, time::Duration};

use serde::{ser::Serializer, Deserialize, Serialize};

use super::account_method::AccountMethod;
#[cfg(feature = "events")]
#[cfg(debug_assertions)]
use crate::events::types::WalletEvent;
use crate::{
    account::{operations::syncing::SyncOptions, types::AccountIdentifier},
    ClientOptions,
};

/// An account to create.
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountToCreate {
    /// The account alias.
    pub alias: Option<String>,
    /// The account coin type.
    pub coin_type: Option<u32>,
}

/// The messages that can be sent to the actor.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "cmd", content = "payload")]
#[allow(clippy::large_enum_variant)]
pub enum MessageType {
    /// Creates an account.
    CreateAccount(Box<AccountToCreate>),
    /// Read account.
    GetAccount(AccountIdentifier),
    /// Read accounts.
    GetAccounts,
    /// Consume an account method.
    CallAccountMethod {
        /// The account identifier.
        #[serde(rename = "accountId")]
        account_id: AccountIdentifier,
        /// The account method to call.
        method: AccountMethod,
    },
    #[cfg(feature = "storage")]
    /// Backup storage.
    Backup {
        /// The backup destination.
        destination: PathBuf,
        /// Stronghold file password.
        password: String,
    },
    // Todo: need to decide if we have an extra method for that or if the options for the account manager alone should
    // be used #[cfg(feature = "storage")]
    // /// Import accounts from storage.
    // RestoreBackup {
    //     /// The path to the backed up storage.
    //     source: String,
    //     /// Stronghold file password.
    //     password: String,
    // },
    #[cfg(feature = "storage")]
    /// Deletes the storage.
    DeleteStorage,
    /// Generates a new mnemonic.
    GenerateMnemonic,
    /// Checks if the given mnemonic is valid.
    VerifyMnemonic(String),
    /// Updates the client options for all accounts.
    SetClientOptions(Box<ClientOptions>),
    /// Get the node information
    GetNodeInfo,
    /// Set the stronghold password.
    SetStrongholdPassword(String),
    /// Store a mnemonic into the Stronghold vault.
    StoreMnemonic(String),
    /// Start background syncing.
    StartBackgroundSync {
        /// Sync options
        options: Option<SyncOptions>,
        /// Interval
        interval: Option<Duration>,
    },
    /// Stop background syncing.
    StopBackgroundSync,
    #[cfg(feature = "events")]
    #[cfg(debug_assertions)]
    EmitTestEvent(WalletEvent),
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageType::CreateAccount(_) => serializer.serialize_unit_variant("MessageType", 1, "CreateAccount"),
            MessageType::GetAccount(_) => serializer.serialize_unit_variant("MessageType", 2, "GetAccount"),
            MessageType::GetAccounts => serializer.serialize_unit_variant("MessageType", 3, "GetAccounts"),
            MessageType::CallAccountMethod { .. } => {
                serializer.serialize_unit_variant("MessageType", 4, "CallAccountMethod")
            }
            MessageType::Backup { .. } => serializer.serialize_unit_variant("MessageType", 5, "Backup"),
            // Todo: need to decide if we have an extra method for that or if the options for the account manager alone
            // should be used MessageType::RestoreBackup { .. } =>
            // serializer.serialize_unit_variant("MessageType", 6, "RestoreBackup"),
            MessageType::GenerateMnemonic => serializer.serialize_unit_variant("MessageType", 7, "GenerateMnemonic"),
            MessageType::VerifyMnemonic(_) => serializer.serialize_unit_variant("MessageType", 8, "VerifyMnemonic"),
            MessageType::DeleteStorage => serializer.serialize_unit_variant("MessageType", 9, "DeleteStorage"),
            MessageType::SetClientOptions(_) => {
                serializer.serialize_unit_variant("MessageType", 10, "SetClientOptions")
            }
            MessageType::GetNodeInfo => serializer.serialize_unit_variant("MessageType", 11, "GetNodeInfo"),
            MessageType::SetStrongholdPassword(_) => {
                serializer.serialize_unit_variant("MessageType", 12, "SetStrongholdPassword")
            }
            MessageType::StoreMnemonic(_) => serializer.serialize_unit_variant("MessageType", 12, "StoreMnemonic"),
            MessageType::StartBackgroundSync { .. } => {
                serializer.serialize_unit_variant("MessageType", 13, "StartBackgroundSync")
            }
            MessageType::StopBackgroundSync => {
                serializer.serialize_unit_variant("MessageType", 14, "StopBackgroundSync")
            }
            #[cfg(feature = "events")]
            #[cfg(debug_assertions)]
            MessageType::EmitTestEvent(_) => serializer.serialize_unit_variant("MessageType", 15, "EmitTestEvent"),
        }
    }
}
