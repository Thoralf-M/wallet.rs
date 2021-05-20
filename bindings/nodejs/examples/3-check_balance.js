/**
 * This example creates a new database and account
 */

require('dotenv').config()

async function run() {
    const { AccountManager, initLogger, addEventListener } = require('@iota/wallet')
    const manager = new AccountManager({
        storagePath: './alice-database'
    })
    initLogger({
        color_enabled: true,
        outputs: [{
            name: './prod23.log',
            level_filter: 'debug'
        }]
    })
    const callback = function (err, data) {
        console.log("data:", data)
    }

    addEventListener("BalanceChange", callback)
    manager.setStrongholdPassword("password")

    const account = manager.getAccount('Alice')

    console.log('Account:', account.alias())

    // Always sync before doing anything with the account
    const synced = await account.sync()
    console.log('Syncing...')

    console.log('Available balance', account.balance().available)
}

run()
