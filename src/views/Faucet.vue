<template>
  <div class="bg-gray-900 w-full flex flex-col h-full">
    <div>
      <h1 class="pt-5 font-bold text-2xl text-white text-center bg-transparent">Faucet</h1>
    </div>
    <div class="mt-5 bg-gray-900 mx-auto px-5 flex-1 h-full md:w-3/5 xl:w-2/5 w-full">
      <div class="">
        <ul class="flex flex-nowrap text-sm font-medium text-center text-gray-500 md:justify-content-center">
          <li class="mr-1">
            <a href="javascript:void(0)" class="w-20 inline-block p-4 hover:text-gray-50" v-bind:class="{ 'text-gray-200': tab === 'USDT' }" @click="onTab('USDT')">USDT</a>
          </li>
          <li class="mr-1">
            <a href="javascript:void(0)" class="w-20 inline-block p-4 hover:text-gray-50" v-bind:class="{ 'text-gray-200': tab === 'BTC' }" @click="onTab('BTC')">BTC</a>
          </li>
          <li class="mr-1">
            <a href="javascript:void(0)" class="w-20 inline-block p-4 hover:text-gray-50" v-bind:class="{ 'text-gray-200': tab === 'BNB' }" @click="onTab('BNB')">BNB</a>
          </li>
        </ul>
      </div>
      <div class="w-full">
        <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only">Search</label>
        <div class="relative">
          <input type="search" id="default-search" class="block p-4 pl-5 w-full text-sm text-gray-900 bg-gray-50 border border-gray-300 focus:ring-blue-500" placeholder="Please paste your account address" required />
          <button class="float-right text-white md:absolute relative md:right-2.5 bottom-2.5 md:mt-0 mt-3 focus:outline-none font-medium rounded-lg text-sm px-4 py-2 bg-gray-800 hover:bg-gray-600 focus:ring-blue-800" @click="onFaucet">Give me Token</button>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
import { mapActions, mapGetters } from 'vuex'

export default {
  name: 'faucet',
  data() {
    return {
      tab: 'USDT',
    }
  },
  computed: {
    ...mapGetters(['market', 'orderbookContract', 'nearConfig', 'currentUser', 'tokens']),
  },
  methods: {
    onTab(tab) {
      this.tab = tab
      console.log(tab)
    },
    async onFaucet() {
      let token = this.tokens[this.tab]
      await token.ft_mint(
        {
          receiver_id: this.currentUser.accountId,
          amount: '500000',
          memo: '',
        },
        this.nearConfig.GAS
      )
    },
  },
}
</script>

<style scoped></style>
