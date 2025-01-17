import type { Address, AddressWithUnspentOutputs } from './address';
import type { OutputData } from './output';
import type { Transaction } from './transaction';

/**
 * Account identifier
 * Could be the account index (number) or account alias (string)
 */
export type AccountId = number | string;

export interface AccountBalance {
    baseCoin: BaseCoinBalance;
    requiredStorageDeposit: string;
    nativeTokens: NativeTokenBalance[];
    nfts: string[];
    aliases: string[];
    foundries: string[];
    /**
     * Outputs with multiple unlock conditions and if they can currently be spent or not. If there is a
     * TimelockUnlockCondition or ExpirationUnlockCondition this can change at any time
     */
    potentiallyLockedOutputs: { [outputId: string]: boolean };
}

export interface AccountSyncOptions {
    /**
     * Specific Bech32 encoded addresses of the account to sync, if addresses are provided,
     * then `address_start_index` will be ignored
     */
    addresses?: string[];
    /**
     * Address index from which to start syncing addresses. 0 by default, using a higher index will be faster because
     * addresses with a lower index will be skipped, but could result in a wrong balance for that reason
     */
    addressStartIndex?: number;
    /**
     * Usually we skip syncing if it's called within a few seconds, because there can only be new changes every 5
     * seconds. But if we change the client options, we need to resync, because the new node could be from a nother
     * network and then we need to check all addresses. This will also ignore `address_start_index` and sync all
     * addresses. Default: false.
     */
    forceSyncing?: boolean;
    /** Checks pending transactions and promotes/reattaches them if necessary.  Default: true. */
    syncPendingTransactions?: boolean;
    /** Specifies if only basic outputs should be synced or also alias and nft outputs. Default: true. */
    syncAliasesAndNfts?: boolean;
    /** Specifies if only basic outputs with an AddressUnlockCondition alone should be synced, will overwrite
     * `syncAliasesAndNfts`. Default: false. */
    syncOnlyMostBasicOutputs?: boolean;
}

export interface AccountMeta {
    index: number;
    coinType: CoinType;
    alias: string;
    publicAddresses: Address[];
    internalAddresses: Address[];
    addressesWithUnspentOutputs: AddressWithUnspentOutputs[];
    outputs: { [outputId: string]: OutputData };
    /** Output IDs of unspent outputs that are currently used as input for transactions */
    lockedOutputs: Set<string>;
    unspentOutputs: { [outputId: string]: OutputData };
    transactions: { [transactionId: string]: Transaction };
    /** Transaction IDs of pending transactions */
    pendingTransactions: Set<string>;
}

export interface BaseCoinBalance {
    total: string;
    available: string;
}

export interface NativeTokenBalance extends BaseCoinBalance {
    id: string
}

export enum CoinType {
    IOTA = 4218,
    Shimmer = 4219,
}

export interface CreateAccountPayload {
    alias?: string;
}
