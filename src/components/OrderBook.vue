<template>
  <div class="flex flex-col bg-gray-900 px-3 py-1 h-full">
    <div class="flex justify-between">
      <span v-bind:class="[option === 'orderbook' ? activeClass : inActiveClass]" @click="onOrderbook"> Order Book</span>
      <span v-bind:class="[option === 'trades' ? activeClass : inActiveClass]" @click="onMarketTrades"> Market Trades</span>
    </div>
    <div v-if="option === 'orderbook'">
      <table class="table-auto w-full">
        <thead class="bg-gray-900">
          <tr class="text-right">
            <th scope="col" class="text-left py-2 text-xs font-medium tracking-wider text-gray-400 uppercase">Size({{ baseToken }})</th>
            <th scope="col" class="py-2 text-xs font-medium tracking-wider text-gray-400 uppercase">Price({{ quoteToken }})</th>
          </tr>
        </thead>
      </table>

      <div v-for="(ask, index) in askToDisplay" :key="index" @click="onAskClick(ask)" class="w-full mt-0.5 relative bg-red-600 text-white h-5 flex text-left text-xs items-center">
        <span class="bg-gray-900 h-5 text-white text-left text-xs items-center flex" :style="{ width:   ask.sizePercent + '%' }">{{ format(ask.size, 2) }}</span>
        <span class="absolute right-0.5">{{ ask.price }}</span>
      </div>
      <div class="mt-2 mb-2 text-green-400 text-center text-xs" v-if="spread > 0">
        {{ spread }}
      </div>
      <div v-for="(bid, index) in bidToDisplay" :key="index" @click="onBidClick(bid)" class="w-full mt-0.5 relative bg-green-500 text-white h-5 flex text-left text-xs items-center">
        <span class="bg-gray-900 h-5 text-white text-left text-xs items-center flex" :style="{ width:   bid.sizePercent + '%' }">{{ format(bid.size, 2) }}</span>
        <span class="absolute right-0.5">{{ bid.price }}</span>
      </div>
    </div>
    <div v-else>
      <MarketHistory :baseToken="orderbookMarket.split('-')[0]" :quoteToken="orderbookMarket.split('-')[1]" />
    </div>
  </div>
</template>

<script>
import { formatNumber } from '../utils/token.js'
import MarketHistory from './MarketHistory.vue'
import { mapGetters } from 'vuex'
export default {
  data() {
    return {
      isVisible: false,
      bidToDisplay: [],
      askToDisplay: [],
      spread: 0,
      depth: 100,
      orderbookMarket: '',
      option: 'orderbook',
      selected: '',
      activeClass: 'text-gray-50',
      inActiveClass: 'text-gray-600',
    }
  },
  props: ['baseToken', 'quoteToken', 'market'],
  components: { MarketHistory },
  computed: {
    ...mapGetters(['market', 'orderbookContract']),
  },
  watch: {
    market: function (newVal, oldVal) {
      this.bidToDisplay = []
      this.askToDisplay = []
      this.spread = 0
      this.orderbookMarket = newVal
      console.log(`order book market change previous: ${oldVal}, now:${newVal}`)
      this.flash = []
    },
  },
  methods: {
    getRowCls(index, ask) {
      if (this.flash[index]) {
        return { size: 'h-[18px] z-10 bg-red-600 flex items-center', price: 'bg-red-600 h-[18px] text-white text-left text-xs items-center flex' }
      }
      return { size: 'h-[18px] z-10 flex items-center', price: `absolute right-0 h-[18px] bg-red-600 items-center flex flex-row-reverse ` }
    },
    getStyle(index, ask) {
      if (this.flash[index]) {
        return 'width: 100%;'
      }
      return `width: ${ask.sizePercent}%`
    },
    format(num, decimal) {
      return formatNumber(num, decimal)
    },
    tab(index) {
      this.num = index
    },
    onBidClick(bid) {
      this.$emit('updateBid', bid)
    },
    onAskClick(ask) {
      this.$emit('updateAsk', ask)
    },
    async tick() {
      let ret = await this.orderbookContract.orderbook()
      //  console.log('orderbook', ret[0], ret[1])
      let ask = {}
      Object.entries(ret[1]).forEach(([_, v]) => (ask[v.price] == null ? (ask[v.price] = v.qty) : (ask[v.price] += v.qty)))
      let asks = []
      Object.entries(ask).forEach(([price, qty]) => asks.push({ size: Number(qty), price: Number(price).toFixed(4) }))
      asks.reverse()
      let bid = {}
      Object.entries(ret[0]).forEach(([_, v]) => (bid[v.price] == null ? (bid[v.price] = v.qty) : (bid[v.price] += v.qty)))
      let bids = []
      Object.entries(bid).forEach(([price, qty]) => bids.push({ size: Number(qty), price: Number(price).toFixed(4) }))

      let sum = (total, { _, size }, index) => (index < this.depth ? total + size : total)
      let totalSize = bids.reduce(sum, 0) + asks.reduce(sum, 0)
      let bidToDisplay = this.getCumulativeOrderbookSide(bids, totalSize, true)
      this.bidToDisplay = bidToDisplay
   
      let askToDisplay = this.getCumulativeOrderbookSide(asks, totalSize, false)
      this.askToDisplay = askToDisplay

      this.spread = formatNumber(askToDisplay[askToDisplay.length - 1]?.price - bidToDisplay[0]?.price, 4)
    },
    getCumulativeOrderbookSide(orders, totalSize, backwards) {
      let cumulative = orders.slice(0, this.depth).reduce((cumulative, { price, size }, i) => {
        const cumulativeSize = (cumulative[i - 1]?.cumulativeSize || 0) + size
        cumulative.push({
          price,
          size,
          cumulativeSize,
          sizePercent: Math.round((cumulativeSize / (totalSize || 1)) * 100),
        })
        return cumulative
      }, [])
      if (backwards) {
        cumulative = cumulative.reverse()
      }
      return cumulative
    },
    onOrderbook() {
      this.option = 'orderbook'
    },
    onMarketTrades() {
      this.option = 'trades'
    },
  },
  mounted() {
    this.timer = setInterval(this.tick, 500)
  },
  beforeUnmount() {
    clearTimeout(this.timer)
  },
}
</script>

<style scoped>
.background {
  width: 100%;

  background-color: gray;
  z-index: 2;
}

.content {
  position: relative;
  z-index: 1;
  right: 0%;
  width: 100%;
  background-color: red;
  text-align: center;
  font-weight: bold;
  font-family: Arial, sans-serif;
}
</style>
