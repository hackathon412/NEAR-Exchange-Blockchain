<template>
  <nav class="flex justify-between bg-transparent z-10">
    <div class="flex ml-5">
      <div class="w-12 h-12 flex" style="">
        <img class="h-10 my-auto" src="https://perp.spin.fi/images/near-wallet-icon.png" />
      </div>
      <div class="ml-10 hidden justify-between items-center w-full md:flex md:w-auto md:order-1" id="mobile-menu-4">
        <ul class="flex flex-col mt-4 md:flex-row md:space-x-8 md:mt-0 font-bold">
          <li v-for="item in navItems" :key="item.label">
            <router-link v-if="item.link !== ''" :to="item.link">
              <a class="block py-2 pr-4 pl-3 text-gray-50 border-b border-gray-100 hover:bg-gray-50 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0">{{ item.label }}</a>
            </router-link>
            <VTooltip v-if="item.link === ''">
              <a class="text-gray-50 border-gray-100 cursor-pointer">{{ item.label }}</a>
              <template #popper>Coming soon ! </template>
            </VTooltip>
          </li>
        </ul>
      </div>
      <span class="ml-3 self-center text-2xl font-semibold whitespace-nowrap text-gray-50 cursor-pointer" @click="$router.push('/')">NEAR Exchange</span>
    </div>
    <ConnectWallet />
    <div class="md:hidden flex items-center pr-3">
      <button class="outline-none mobile-menu-button" @click="toggleMenu">
        <svg class="w-6 h-6" :class="{ 'text-blue-500': menuVisible, 'text-gray-500': !menuVisible }" x-show="!showMenu" fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
          <path d="M4 6h16M4 12h16M4 18h16"></path>
        </svg>
      </button>
    </div>
  </nav>
  <div v-if="menuVisible" id="mobile-menu" class="origin-top-right right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-gray-800 ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="user-menu-button" tabindex="-1">
    <ul>
      <li v-for="item in navItems" :key="item.label">
        <router-link :to="item.link">
          <a class="block px-4 py-2 text-md font-bold text-gray-200 hover:bg-gray-700 hover:text-white" role="menuitem" tabindex="-1" id="user-menu-item-0">{{ item.label }}</a>
        </router-link>
      </li>
    </ul>
  </div>
</template>

<script>
import ConnectWallet from './ConnectWallet.vue'
export default {
  data() {
    return {
      btnTxt: 'Connect Wallet',
      isVisible: false,
      menuVisible: false,
      navItems: [
        { label: 'Trade', link: '/trade' },
        { label: 'Faucet', link: '/faucet' },
      ],
    }
  },
  components: {
    ConnectWallet,
  },
  created() {},
  methods: {
    async onConnectWallet() {
      await connect()
      console.log(walletGlobal.account)
      this.btnTxt = walletGlobal.account
    },
    tab(index) {
      this.num = index
    },
    toggleMenu() {
      this.menuVisible = !this.menuVisible
    },
  },
}
</script>

<style scoped>
#mobile-menu {
  top: 49px;
  z-index: 1;
  width: 100%;
}
</style>
