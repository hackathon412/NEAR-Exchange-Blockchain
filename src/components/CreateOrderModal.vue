<template>
  <div tabindex="-1" aria-hidden="false" class="overflow-x-hidden overflow-y-auto inset-0 fixed z-50 h-modal justify-center items-center pt-40">
    <div class="relative w-auto my-6 mx-auto max-w-sm">
      <div class="relative rounded-lg shadow bg-gray-800">
        <button type="button" class="absolute top-3 right-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center" data-modal-toggle="crypto-modal" @click="onClose">
          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
        </button>

        <div class="py-4 px-6 rounded-t border-b border-gray-600">
          <h3 class="text-base font-semibold text-gray-200 lg:text-xl">Create Order</h3>
        </div>

        <div class="pt-4 pb-6 px-6 text-white text-sm">
          <div class="bg-gray-900 p-2">
            <div class="flex flex-row justify-between">
              <p class="text-gray-500 text-xs">Order Type</p>
              <p class="text-gray-300">Market</p>
            </div>
            <div class="flex flex-row justify-between">
              <p class="text-gray-500 text-xs">Side</p>
              <p class="text-gray-300">Sell</p>
            </div>
            <div class="flex flex-row justify-between">
              <p class="text-gray-500 text-xs">Size</p>
              <p class="text-gray-300">123.232</p>
            </div>
          </div>
        </div>

        <div class="pt-2 pb-3 px-6 text-white text-sm">
          <div class="bg-gray-900 p-2">
            <div class="flex flex-row justify-between items-center">
              <p class="text-gray-500 text-xs">ETH Balance</p>
              <p class="text-gray-300">123.0023</p>
            </div>
            <div class="flex flex-row justify-between items-center">
              <p class="text-gray-500 text-xs">Transaction Fee</p>
              <p class="text-gray-300">0.000 USDT</p>
            </div>
            <div class="flex flex-row justify-between items-center">
              <p class="text-gray-500 text-xs">Estimated Gas Fee</p>
              <p class="text-gray-300">123.000 USDT</p>
            </div>
            <div class="flex flex-row justify-between items-center">
              <p class="text-gray-500 text-xs">Total</p>
              <p class="text-gray-300">123.000 USDT</p>
            </div>
          </div>
        </div>

        <div class="mt-10 px-6 flex justify-center items-center pb-4">
          <button class="py-2 w-full bg-gray-500 hover:bg-gray-400 text-gray-50" @click="onDeposit">Submit</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {}
  },
  components: {},
  computed: {
    // ...mapGetters(['isConnected', 'account']),
  },

  created() {},
  methods: {},
}
</script>

<style scoped></style>
