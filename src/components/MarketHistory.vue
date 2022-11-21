<template>
  <div class="mt-2">
    <div class="flex flex-col" style="height:80vh">
      <div class="  bg-gray-900 overflow-y-auto h-full">
        <table class="w-full">
          <thead class="">
            <tr>
              <th scope="col" class="text-xs font-medium tracking-wider text-left text-gray-400 uppercase">Price({{ quoteToken }})</th>
              <th scope="col" class="text-xs font-medium tracking-wider text-center text-gray-400 uppercase">Size({{ baseToken }})</th>
              <th scope="col" class="text-xs font-medium tracking-wider text-right text-gray-400 uppercase">Time</th>
            </tr>
          </thead>
          <tbody>
            <tr class="" v-for="item in history" :key="item.id">
              <td class="text-xs text-red-400 whitespace-nowrap text-left" v-bind:class="{ 'text-green-400': item.TakerSide === 'buy' }">{{ item.PriceInDecimal }}</td>
              <td class="text-xs text-white whitespace-nowrap text-center">{{ item.AmountInDecimal }}</td>
              <td class="text-xs text-white whitespace-nowrap text-right">{{ dateTime(item.CreateTime) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
<script>
 
import moment from 'moment'

export default {
  props: ['baseToken', 'quoteToken'],
  data() {
    return {
      history: null,
    }
  },
  methods: {
    tick() {
      marketHistory({
        marketID: `${this.baseToken}-${this.quoteToken}`,
      }).then((ret) => {
        console.log('==> marketHistory ret:', ret)
        this.history = ret
      })
    },
    dateTime(value) {
      return moment(value * 1000).format('MM-DD HH:mm:ss')
    },
  },
  mounted() {
    this.timer = setInterval(this.tick, 2000)
  },
  beforeUnmount() {
    clearTimeout(this.timer)
  },
}
</script>
<style></style>
