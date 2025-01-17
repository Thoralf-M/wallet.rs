import type { AccountId } from '../account';
import type {
    __BuildAliasOutputMethod__,
    __BuildBasicOutputMethod__,
    __BuildFoundryOutputMethod__,
    __BuildNftOutputMethod__,
    __ClaimOutputsMethod__,
    __ConsolidateOutputsMethod__,
    __GenerateAddressesMethod__,
    __GetBalanceMethod__,
    __GetOutputMethod__,
    __GetFoundryOutputMethod__,
    __GetOutputsWithAdditionalUnlockConditionsMethod__,
    __GetTransactionMethod__,
    __ListAddressesMethod__,
    __ListAddressesWithUnspentOutputsMethod__,
    __ListOutputsMethod__,
    __ListPendingTransactionsMethod__,
    __ListTransactionsMethod__,
    __ListUnspentOutputsMethod__,
    __MinimumRequiredStorageDepositMethod__,
    __MintNativeTokenMethod__,
    __MintNftsMethod__,
    __PrepareOutputMethod__,
    __PrepareSendAmountMethod__,
    __PrepareTransactionMethod__,
    __SendAmountMethod__,
    __SendMicroTransactionMethod__,
    __SendNativeTokensMethod__,
    __SendNftMethod__,
    __SendOutputsMethod__,
    __SetAliasMethod__,
    __SignTransactionEssenceMethod__,
    __SubmitAndStoreTransactionMethod__,
    __SyncAccountMethod__,
    __TryClaimOutputsMethod__,
} from './account';
import type {
    __BackupMessage__,
    __Bech32ToHex__,
    __ChangeStrongholdPasswordMessage__,
    __ClearStrongholdPasswordMessage__,
    __CreateAccountMessage__,
    __DeleteAccountsAndDatabaseMessage__,
    __EmitTestEventMessage__,
    __GenerateMnemonicMessage__,
    __GetAccountMessage__,
    __GetAccountsMessage__,
    __GetLedgerStatusMessage__,
    __GetNodeInfoMessage__,
    __HexToBech32__,
    __IsStrongholdPasswordAvailableMessage__,
    __RecoverAccountsMessage__,
    __RemoveLatestAccountMessage__,
    __RestoreBackupMessage__,
    __SetClientOptionsMessage__,
    __SetStrongholdPasswordClearIntervalMessage__,
    __SetStrongholdPasswordMessage__,
    __StartBackgroundSyncMessage__,
    __StopBackgroundSyncMessage__,
    __StoreMnemonicMessage__,
    __VerifyMnemonicMessage__,
} from './accountManager';

export type __AccountMethod__ =
    | __BuildAliasOutputMethod__
    | __BuildBasicOutputMethod__
    | __BuildFoundryOutputMethod__
    | __BuildNftOutputMethod__
    | __ClaimOutputsMethod__
    | __ConsolidateOutputsMethod__
    | __GenerateAddressesMethod__
    | __GetBalanceMethod__
    | __GetOutputMethod__
    | __GetFoundryOutputMethod__
    | __GetOutputsWithAdditionalUnlockConditionsMethod__
    | __GetTransactionMethod__
    | __ListAddressesMethod__
    | __ListAddressesWithUnspentOutputsMethod__
    | __ListOutputsMethod__
    | __ListPendingTransactionsMethod__
    | __ListTransactionsMethod__
    | __ListUnspentOutputsMethod__
    | __MinimumRequiredStorageDepositMethod__
    | __MintNativeTokenMethod__
    | __MintNftsMethod__
    | __PrepareOutputMethod__
    | __PrepareSendAmountMethod__
    | __PrepareTransactionMethod__
    | __SendAmountMethod__
    | __SendMicroTransactionMethod__
    | __SendNativeTokensMethod__
    | __SendNftMethod__
    | __SendOutputsMethod__
    | __SetAliasMethod__
    | __SignTransactionEssenceMethod__
    | __SubmitAndStoreTransactionMethod__
    | __SyncAccountMethod__
    | __TryClaimOutputsMethod__;

export type __CallAccountMethodMessage__ = {
    cmd: 'CallAccountMethod';
    payload: {
        accountId: AccountId;
        method: __AccountMethod__;
    };
};

export type __Message__ =
    | __BackupMessage__
    | __Bech32ToHex__
    | __CallAccountMethodMessage__
    | __ChangeStrongholdPasswordMessage__
    | __ClearStrongholdPasswordMessage__
    | __CreateAccountMessage__
    | __DeleteAccountsAndDatabaseMessage__
    | __EmitTestEventMessage__
    | __GenerateMnemonicMessage__
    | __GetAccountMessage__
    | __GetAccountsMessage__
    | __GetLedgerStatusMessage__
    | __GetNodeInfoMessage__
    | __HexToBech32__
    | __IsStrongholdPasswordAvailableMessage__
    | __RecoverAccountsMessage__
    | __RemoveLatestAccountMessage__
    | __RestoreBackupMessage__
    | __SetClientOptionsMessage__
    | __SetStrongholdPasswordClearIntervalMessage__
    | __SetStrongholdPasswordMessage__
    | __StartBackgroundSyncMessage__
    | __StopBackgroundSyncMessage__
    | __StoreMnemonicMessage__
    | __VerifyMnemonicMessage__;
