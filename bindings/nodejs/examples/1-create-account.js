/**
 * This example creates a new database and account
 */

require('dotenv').config();
const { CoinType } = require('../out/types');
const { AccountManager } = require('@iota/wallet');

async function run() {
    try {
        const manager = await createAccountManager();

        // The coin type only needs to be set on the first account
        const account = await manager.createAccount({
            alias: 'Alice',
        });
        console.log('Account created:', account);

        const secondAccount = await manager.createAccount({
            alias: 'Bob',
        });
        console.log('Account created:', secondAccount);
    } catch (error) {
        console.log('Error: ' + error);
    }
    process.exit(0);
}

async function createAccountManager() {
    const accountManagerOptions = {
        storagePath: './alice-database',
        clientOptions: {
            nodes: [
                {
                    url: 'https://firefly.h.chrysalis-devnet.iota.cafe/',
                },
            ],
            localPow: true,
        },
        coinType: CoinType.Shimmer,
        secretManager: {
            Stronghold: {
                snapshotPath: `./wallet.stronghold`,
                password: `${process.env.SH_PASSWORD}`,
            },
        },
    };

    const manager = new AccountManager(accountManagerOptions);
    await manager.storeMnemonic(process.env.MNEMONIC);
    return manager;
}

run();
