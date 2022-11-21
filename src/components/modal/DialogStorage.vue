<template>
  <TransitionRoot appear :show="isOpen" as="template">
    <Dialog as="div" class="relative z-20">
      <TransitionChild as="template" enter="duration-300 ease-out" enter-from="opacity-0" enter-to="opacity-100" leave="duration-200 ease-in" leave-from="opacity-100" leave-to="opacity-0">
        <div class="fixed inset-0 bg-black bg-opacity-25" />
      </TransitionChild>

      <div class="fixed inset-0">
        <div class="flex min-h-full items-center justify-center px-4 text-center w-96 mx-auto">
          <TransitionChild as="template" enter="duration-300 ease-out" enter-from="opacity-0 scale-95" enter-to="opacity-100 scale-100" leave="duration-200 ease-in" leave-from="opacity-100 scale-100" leave-to="opacity-0 scale-95">
            <DialogPanel class="w-full max-w-md transform rounded-md bg-gray-800 p-6 text-left align-middle shadow-xl transition-all">
              <DialogTitle as="h3" class="text-lg font-medium leading-6 text-gray-900">
                <div class="text-white">Storage Deposit</div>
                <button></button>
                <button type="button" class="absolute top-3 right-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-md text-sm p-1.5 ml-auto inline-flex items-center" data-modal-toggle="crypto-modal" @click="onClose">
                  <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                </button>
              </DialogTitle>

              <div class="mt-10 flex justify-center items-center pb-4">
                <button class="py-2 w-full bg-gray-500 hover:bg-gray-400 text-gray-50" @click="onRegister">Register Account</button>
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>

<script>
import { TransitionRoot, TransitionChild, Dialog, DialogPanel, DialogTitle } from '@headlessui/vue'
import { mapGetters } from 'vuex'
import tokenList from '../../config/tokenList'
import dropdown from './DropDown.vue'
import { formatNumber } from '../../utils/token'

export default {
  name: 'DialogStorage',
  components: {
    TransitionRoot,
    TransitionChild,
    Dialog,
    DialogPanel,
    DialogTitle,
    dropdown,
  },
  computed: {
    ...mapGetters(['account', 'orderbookContract', 'currentUser', 'nearConfig']),
  },
  props: ['isOpen'],
  data() {
    return {
      selectedToken: {
        name: 'BTC',
        balance: '',
        maxBalance: '',
      },
    }
  },
  methods: {
    onClose() {
      this.$emit('close')
    },
    async onMax() {
      this.selectedToken.maxBalance = formatNumber(this.selectedToken.balance, 4)
    },
    async onRegister() {
      console.log(this.nearConfig)
      await this.orderbookContract.storage_deposit(
        {
          account_id: this.currentUser.accountId,
          registration_only: false,
        },
        this.nearConfig.GAS,
        '500000000000000000000000'
      )
    },
    tokens() {
      return tokenList.tokens.map((currElement, index) => {
        return {
          name: currElement.symbol,
          id: index,
        }
      })
    },
    format(val) {
      return formatNumber(val, 4)
    },
  },
}
</script>
