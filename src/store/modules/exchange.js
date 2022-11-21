import * as nearApi from 'near-api-js'
import getConfigurations from '../../config'

const state = {
  currentUser: null,
  wallet: null,
  nearConfig: null,
  orderbookContract: null,
  account: 'Connect Wallet',
}
const mutations = {
  setupNear(state, payload) {
    state.currentUser = payload.currentUser
    state.wallet = payload.wallet
    state.nearConfig = payload.nearConfig
    state.orderbookContract = payload.orderbookContract
    state.tokens = payload.tokens
  },
}

const actions = {
  async initNear({ commit }) {
    const nearConfig = getConfigurations('testnet')
    const near = await nearApi.connect({
      deps: {
        keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore(),
      },
      ...nearConfig,
    })

    const wallet = new nearApi.WalletConnection(near)
    let currentUser
    if (wallet.getAccountId()) {
      currentUser = {
        accountId: wallet.getAccountId(),
        balance: (await wallet.account().state()).amount,
        balanceInNear: (await wallet.account().state()).amount / 10 ** 24,
      }
    }
    console.log(currentUser)
    console.log('orderbookContract ', nearConfig.orderbookContract)
    // init orderbook contract
    const orderbookContract = await new nearApi.Contract(wallet.account(), nearConfig.orderbookContract, {
      viewMethods: ['orderbook', 'ft_deposits_of', 'storage_balance_bounds', 'storage_balance_of', 'ft_deposits_of_asset'],
      changeMethods: ['new_limit_order', 'place_order', 'ft_withdraw', 'storage_deposit'],
      sender: wallet.getAccountId(),
    })

    // init FT contract
    let tokens = {}
    nearConfig.tokens.map(async (item) => {
      let token = await new nearApi.Contract(wallet.account(), item.address, {
        viewMethods: ['ft_balance_of'],
        changeMethods: ['ft_mint', 'ft_transfer', 'ft_transfer_call'],
        sender: wallet.getAccountId(),
      })
      tokens[item.symbol] = token
    })

    // let storageBalance = await orderbookContract.storage_balance_of({ account_id: currentUser.accountId })
    // console.log('storageBalance**', storageBalance)

    let storageBounds = await orderbookContract.storage_balance_bounds()
    console.log('storageBounds**', storageBounds)

    // Commit and send to mutation.
    commit('setupNear', {
      currentUser,
      wallet,
      nearConfig,
      orderbookContract,
      tokens,
    })
  },
}

const getters = {
  contract: (state) => state.contract,
  currentUser: (state) => state.currentUser,
  wallet: (state) => state.wallet,
  nearConfig: (state) => state.nearConfig,
  orderbookContract: (state) => state.orderbookContract,
  ftContract: (state) => state.ftContract,
  USDC: (state) => state.USDC,
  tokens: (state) => state.tokens,
}

export default {
  state,
  mutations,
  actions,
  getters,
}
