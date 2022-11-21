<template>
  <div class="flex justify-items-center items-center">
    <router-link to="" @click="onConnectWallet" v-if="showBtn">
      <span class="px-4 py-1 rounded mr-4 text-gray-50 hover:text-white hover:bg-gray-500 font-bold">{{ btnTxt }}</span>
    </router-link>
    <WalletModal :showWallet="showModal" @close="showModal = false" />
  </div>
</template>
<script>
import { mapGetters } from 'vuex'
import WalletModal from './WalletModal.vue'
export default {
  name: 'ConnectWallet',
  data() {
    return {
      showBtn: true,
      btnTxt: 'Connect Wallet',
      showModal: false,
    }
  },
  components: {
    WalletModal,
  },
  computed: {
    ...mapGetters(['isConnected', 'account', 'currentUser', 'wallet']),
  },
  created() {
    console.log('connect button crated', this.wallet)
  },
  watch: {
    $route(to, from) {
      this.showBtn = true
      if (to.fullPath === '/') {
        this.showBtn = false
      }
    },
    currentUser: function () {
      if (this.currentUser) {
        let { accountId } = this.currentUser
        console.log('accountId', accountId)
        this.btnTxt = accountId
      }
    },
  },
  methods: {
    onConnectWallet() {
      if (this.btnTxt !== 'Connect Wallet') {
        this.wallet.signOut()
        window.location.replace(window.location.origin + window.location.pathname)
      } else {
        this.showModal = true
      }
    },
    formatAddress(address) {
      if (address.includes('Connect')) {
        return address
      }
      return `${address.substring(0, 5)}...${address.substring(address.length - 5)}`
    },
    disConnect() {},
  },
}
</script>
<style scoped></style>
