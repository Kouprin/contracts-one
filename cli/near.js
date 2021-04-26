const nearAPI = require('near-api-js')

function getConfig (env) {
  switch (env) {
    case 'mainnet':
      return {
        networkId: 'mainnet',
        nodeUrl: 'https://rpc.mainnet.near.org',
        // TODO
        contractName: '',
        walletUrl: 'https://wallet.near.org',
        helperUrl: 'https://helper.mainnet.near.org'
      }
    case '':
    case 'testnet':
      return {
        networkId: 'testnet',
        nodeUrl: 'https://rpc.testnet.near.org',
        contractName: 'dev-1618917933127-5935675',
        walletUrl: 'https://wallet.testnet.near.org',
        helperUrl: 'https://helper.testnet.near.org',
        keyStore: []
      }
  }
}

async function nearContractView (env) {
  const config = getConfig(env)
  const near = await nearAPI.connect(config)
  const account = await near.account(config.contractName)
  return new nearAPI.Contract(
    account,
    config.contractName, {
      viewMethods: ['get_contract_source_code'],
      changeMethods: []
    })
}

module.exports = {
  getConfig,
  nearContractView,
  nearAPI
}
