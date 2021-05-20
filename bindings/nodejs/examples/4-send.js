/**
 * This example sends IOTA Toens to an address.
 */

require('dotenv').config();

async function run() {
    const { AccountManager, initLogger, addEventListener } = require('@iota/wallet')
    const manager = new AccountManager({
        storagePath: './alice-database',
        syncSpentOutputs: false,
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

    console.log('alias', account.alias())
    console.log('syncing...')
    const synced = await account.sync()
    console.log('available balance', account.balance().available)

    //TODO: Replace with the address of your choice!
    const addr = 'atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r'
    const amount = 1000000

    const node_response = await account.send(
        addr,
        amount
    )

    console.log(`Check your message on https://explorer.iota.org/chrysalis/message/${node_response.id}`)
    await new Promise(resolve => setTimeout(resolve, 300000));
    const synced2 = await account.sync()
}

run()
