<template>
  <div class="flex flex-col w-full h-full">
    <TradeBar />
    <div class="flex flex-1 bg-gray-900 md:flex-row flex-col-reverse md:gap-0 gap-12">
      <div class="md:w-1/5 2xl:w-1/6 w-full border-black border-r">
        <div class="flex flex-col px-0.5 h-full">
          <div class="bg-gray-900 h-full">
            <div class="p-2 bg-gray-900 h-full">
              <ul class="flex rounded-lg divide-gray-200 shadow sm:flex">
                <li class="w-full">
                  <button class="inline-block relative py-2 px-4 w-full text-sm font-medium text-center text-gray-50 bg-gray-800 hover:text-gray-700 hover:bg-gray-600 focus:bg-green-300" @click="toggleTabs('buy')" v-bind:class="{ 'bg-green-300': tab === 'buy' }">BUY</button>
                </li>
                <li class="w-full">
                  <button class="inline-block relative py-2 px-4 w-full text-sm font-medium text-center text-gray-50 bg-gray-800 hover:text-gray-700 hover:bg-gray-600 focus:bg-red-400" @click="toggleTabs('sell')" v-bind:class="{ 'bg-red-400': tab === 'sell' }">SELL</button>
                </li>
              </ul>
              <!-- order type -->
              <div class="pt-2 text-sm font-medium text-center text-gray-500 border-b border-gray-600">
                <ul class="flex flex-wrap -mb-px">
                  <li class="mr-2">
                    <a
                      href="javascript:void(0)"
                      class="inline-block p-2 rounded-t-lg border-b-2 border-transparent hover:text-gray-300 hover:border-blue-600"
                      @click="tabOrderType('market')"
                      v-bind:class="{
                        'border-blue-600 text-gray-300': orderType === 'market',
                      }"
                      >Market</a
                    >
                  </li>
                  <li class="mr-2">
                    <a
                      href="javascript:void(0)"
                      class="inline-block p-2 rounded-t-lg border-b-2 border-transparent hover:text-gray-300 hover:border-blue-600"
                      @click="tabOrderType('limit')"
                      v-bind:class="{
                        'border-blue-600 text-gray-300': orderType === 'limit',
                      }"
                      >Limit</a
                    >
                  </li>
                </ul>
              </div>
              <!-- order type -->

              <!-- market order tab  -->
              <div v-if="orderType === 'market'">
                <div class="mt-3 flex items-center rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500">
                  <span class="w-3/12 text-gray-200 border border-gray-600 py-2 sm:text-sm rounded-l text-center"> Price </span>
                  <div class="relative w-9/12 flex items-center">
                    <input type="text" class="w-full absolute block pr-12 py-2 border sm:text-sm text-gray-50 border-gray-500 text-center bg-gray-800" disabled placeholder="Market Price" />
                    <div class="right-2 text-gray-400 text-sm">
                      {{ quoteToken.name }}
                    </div>
                  </div>
                </div>

                <div class="mt-3 flex items-center rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500">
                  <span class="w-3/12 text-gray-200 border border-gray-600 py-2 sm:text-sm rounded-l text-center"> Size </span>
                  <div class="relative w-9/12 flex items-center">
                    <input type="number" class="w-full absolute block pr-12 py-2 border sm:text-sm text-gray-50 border-gray-500 text-center bg-gray-800" step="0.001" v-model="amount" :min="0" oninput="if(value<0)value=0" />
                    <div class="absolute right-2 text-gray-400 text-sm">
                      {{ tab === 'sell' && orderType === 'market' ? baseToken.name : quoteToken.name }}
                    </div>
                  </div>
                </div>
              </div>
              <!-- market order tab  -->
              <!-- limit order tab  -->
              <div v-if="orderType === 'limit'">
                <div class="mt-3 flex items-center rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500">
                  <span class="w-3/12 text-gray-200 border border-gray-600 py-2 sm:text-sm rounded-l text-center"> Price </span>
                  <div class="relative w-9/12 flex items-center">
                    <input type="number" class="w-full absolute block pr-12 py-2 border sm:text-sm text-gray-50 border-gray-500 text-center bg-gray-800" step="0.001" v-model="price" :min="0" oninput="if(value<0)value=0" />
                    <div class="absolute right-2 text-gray-400 text-sm">
                      {{ quoteToken.name }}
                    </div>
                  </div>
                </div>
                <div class="mt-3 flex items-center rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500">
                  <span class="w-3/12 text-gray-200 border border-gray-600 py-2 sm:text-sm rounded-l text-center"> Size </span>
                  <div class="relative w-9/12 flex items-center">
                    <input type="number" class="w-full absolute block pr-12 py-2 border sm:text-sm text-gray-50 border-gray-500 text-center bg-gray-800" step="0.001" v-model="amount" :min="0" oninput="if(value<0)value=0" />
                    <div class="absolute right-2 text-gray-400 text-sm">
                      {{ baseToken.name }}
                    </div>
                  </div>
                </div>
              </div>
              <!-- limit order tab  -->
              <div class="mt-5 flex justify-center items-center">
                <button class="py-2 w-full bg-gray-800 hover:bg-gray-600 text-gray-50" @click="onTrade">
                  {{ opBtnTxt }}
                </button>
              </div>
              <Assets />
            </div>
          </div>
        </div>
      </div>

      <div class="md:w-1/5 2xl:w-1/6 w-full border-black border-r">
        <OrderBook @updateBid="onBidUpdate" @updateAsk="onAskUpdate" :baseToken="baseToken.name" :quoteToken="quoteToken.name" :market="market" />
      </div>

      <div class="flex flex-col md:w-3/5 2xl:w-4/6 w-full h-full">
        <TradingView :market="market" class="h-[51vh]" />
        <TradeTab :market="market" class="flex-1" />
      </div>
    </div>
    <CreateOrderModal v-if="showCreateOrder" @close="showCreateModal = false" />
    <DialogStorage :isOpen="showStorage" @close="showStorage = false" ref="storage" />
    <TradeFooter />
  </div>
</template>

<script>
import TradingView from '../components/TradingView.vue'
import TradeTab from '../components/UserTrade.vue'
import OrderBook from '../components/OrderBook.vue'
import TradeFooter from '../components/footer/TradeFooter.vue'
import WalletModal from '../components/WalletModal.vue'
import Volume from '../components/Volume.vue'
import Assets from '../components/Assets.vue'
import TradeBar from '../components/TradeBar.vue'
import { mapActions, mapGetters } from 'vuex'
import { createToast } from 'mosha-vue-toastify'
import CreateOrderModal from '../components/CreateOrderModal.vue'
import { getMarketPrice, formatNumber } from '../utils/token'
import DialogStorage from '../components/modal/DialogStorage.vue'

export default {
  name: 'Trade',
  components: {
    TradingView,
    TradeTab,
    OrderBook,
    TradeFooter,
    WalletModal,
    Volume,
    Assets,
    TradeBar,
    CreateOrderModal,
    DialogStorage,
  },
  computed: {
    ...mapGetters(['market', 'orderbookContract', 'currentUser', 'nearConfig']),
  },
  data() {
    return {
      tab: 'buy',
      pools: [],
      opBtnTxt: 'BUY',
      price: 0,
      amount: 0,
      orderType: 'market',
      timer: '',
      baseToken: {
        name: '',
      },
      quoteToken: {
        name: '',
      },
      showCreateOrder: false,
      showStorage: false,
      volume: 0,
      marketPrice: 0,
    }
  },
  watch: {
    market: async function (newVal, _) {
      this.baseToken.name = newVal.split('-')[0]
      this.quoteToken.name = newVal.split('-')[1]
      this.market = newVal
      this.price = 0
      this.amount = 0
      this.volume = 0
    },
    price: function (price, _) {
      if (this.orderType === 'market') {
        this.volume = formatNumber(this.amount, 0)
      } else {
        this.volume = formatNumber(this.price * this.amount, 0)
      }
    },
    amount: function (amount, _) {
      if (this.orderType === 'market') {
        this.volume = formatNumber(this.amount, 0)
      } else {
        this.volume = formatNumber(this.price * amount, 0)
      }
    },
  },
  methods: {
    async trade(side, orderType) {
      console.log('account:', side, orderType)
      console.log('price:', this.price, 'amount:', this.amount)

      if (orderType === 'market') {
        if (this.amount == 0) {
          createToast(
            { title: '', description: 'Please input amount' },
            {
              type: 'danger',
              showIcon: true,
              position: 'top-center',
              timeout: 5000,
            }
          )
          return
        }

        this.price = await getMarketPrice(this.baseToken.name)
        console.log('market price:', this.price)
      } else {
        if (this.price == 0) {
          createToast(
            { title: '', description: 'Please input price' },
            {
              type: 'danger',
              showIcon: true,
              position: 'top-center',
              timeout: 5000,
            }
          )
          return
        }

        if (this.amount == 0) {
          createToast(
            { title: '', description: 'Please input amount' },
            {
              type: 'danger',
              showIcon: true,
              position: 'top-center',
              timeout: 5000,
            }
          )
          return
        }
      }
      let orderSide = this.tab == 'buy' ? 'Bid' : 'Ask'

      console.log('place_order', {
        req: {
          market: 'BTC-USDT',
          side: orderSide,
          price: Number(formatNumber(this.price, 0)),
          qty: Number(formatNumber(this.amount, 0)),
          order_type: this.orderType,
          account: this.currentUser.accountId,
        },
      })
      await this.orderbookContract.place_order(
        {
          req: {
            market: 'BTC-USDT',
            side: orderSide,
            price: Number(formatNumber(this.price, 0)),
            qty: Number(formatNumber(this.amount, 0)),
            order_type: this.orderType,
          },
        },
        this.nearConfig.GAS
      )
    },
    async onTrade() {
      console.log('onTrade', this.currentUser)

      if (this.currentUser.accountId === '') {
        createToast(
          { title: '', description: 'Please connect wallet first' },
          {
            type: 'danger',
            showIcon: true,
            position: 'top-center',
            timeout: 5000,
          }
        )
        return
      }

      this.trade(this.tab, this.orderType)
    },
    toggleTabs(tab) {
      this.tab = tab
      this.price = 0
      this.volUnit = this.quoteToken.name
      this.amount = 0

      if (tab === 'buy') {
        this.opBtnTxt = 'BUY'
      } else {
        this.opBtnTxt = 'SELL'
        if (this.orderType === 'market') {
          this.volUnit = this.baseToken.name
        }
      }
    },
    updateSelectMarket(market) {
      // console.log('updateSelectMarket', market)
      this.market = market
      let tokens = market.split('-')
      this.baseToken.name = tokens[0]
      this.quoteToken.name = tokens[1]
    },
    onBidUpdate(bid) {
      // console.log('onBidUpdate', bid)
      this.price = bid.price
      this.amount = bid.size
    },
    onAskUpdate(ask) {
      // console.log('onAskUpdate', ask)
      this.price = ask.price
      this.amount = ask.size
    },
    async tabOrderType(orderType) {
      this.price = 0
      this.amount = 0
      this.volUnit = this.quoteToken.name
      if (orderType === 'market') {
        if (this.tab === 'buy') {
          this.volUnit = this.baseToken.name
        }
      }

      this.orderType = orderType
    },
    updatePercent(percent) {
      let vol = this.price * this.amount * percent * 0.01
      this.volume = formatNumber(vol, 4)
    },
  },
  async mounted() {
    let tokens = this.market.split('-')
    this.baseToken.name = tokens[0]
    this.quoteToken.name = tokens[1]
    // this.marketPrice = await getMarketPrice(this.baseToken.name)
  },

  async created() {
    await this.$store.dispatch('initNear')
    let storageBalance = await this.orderbookContract.storage_balance_of({ account_id: this.currentUser.accountId })
    if (storageBalance == null) {
      this.showStorage = true
    }
  },
}
</script>
<style scoped></style>
