<template>
  <div class="flex justify-center items-center -z-10 bg-transparent h-full">
    <Menu as="div" class="relative inline-block text-left z-10 bg-transparent my-auto w-full">
      <div class="mx-auto">
        <MenuButton class="mx-auto inline-flex w-full justify-center rounded-md py-2 font-bold text-white hover:bg-opacity-30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75">
          <div class="flex flex-row h-6 justify-center">
            <img class="px-2" :src="tokenLogoURI" alt="" />
            <span class="ml-1"> {{ market }}</span>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" class="default-transition h-6 w-6 rotate-360 transform ml-4"><path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
          </div>
        </MenuButton>
      </div>

      <transition enter-active-class="transition duration-100 ease-out" enter-from-class="transform scale-95 opacity-0" enter-to-class="transform scale-100 opacity-100" leave-active-class="transition duration-75 ease-in" leave-from-class="transform scale-100 opacity-100" leave-to-class="transform scale-95 opacity-0">
        <MenuItems class="absolute w-full mt-2 divide-y divide-gray-100 bg-gray-800 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
          <div class="px-1 py-1">
            <MenuItem v-for="item in marketList" :key="item.id" v-slot="{ active }" class="flex flex-row justify-center">
              <button @click="selectMarket(item.market)" :class="[active ? 'bg-gray-500 text-white' : 'text-gray-900', 'group flex w-full items-center text-center  px-2 py-2 ']">
                <img class="h-6 mr-3" :src="getTokenLogo(item.market)" alt="" />
                <span class="text-white">{{ item.market }}</span>
              </button>
            </MenuItem>
          </div>
        </MenuItems>
      </transition>
    </Menu>
  </div>
</template>

<script>
import { Menu, MenuButton, MenuItems, MenuItem } from '@headlessui/vue'
import marketList from '../config/market_list.json'
import { mapActions, mapGetters } from 'vuex'
import { getTokenCfg } from '../utils/token'

export default {
  data() {
    return {
      btnTxt: 'Connect Wallet',
      market: '',
      marketList: marketList,
      tokenLogoURI: 'https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1547033579',
    }
  },
  components: {
    Menu,
    MenuButton,
    MenuItems,
    MenuItem,
  },
  methods: {
    ...mapActions(['updateMarket']),
    getTokenLogo(market) {
      return getTokenCfg(market.split('-')[0]).logoURI
    },
    selectMarket(market) {
      this.dropdownPopoverShow = false
      this.market = market

      this.updateMarket(market)
      this.tokenLogoURI = getTokenCfg(market.split('-')[0]).logoURI
    },
  },
  mounted() {
    this.market = marketList[0].market
    this.updateMarket(this.market)
    this.tokenLogoURI = getTokenCfg(this.market.split('-')[0]).logoURI
  },
  beforeUnmount() {
    this.updateMarket('')
  },
}
</script>
