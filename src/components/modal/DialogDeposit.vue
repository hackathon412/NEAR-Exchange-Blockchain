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
                <div class="text-white">Wallet Asset</div>
                <button type="button" class="absolute top-3 right-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-md text-sm p-1.5 ml-auto inline-flex items-center" data-modal-toggle="crypto-modal" @click="onClose">
                  <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                </button>
              </DialogTitle>

              <div class="pt-4 pb-6">
                <dropdown class="my-dropdown-toggle" :options="tokenList()" :selected="selectedToken" v-on:updateOption="onDropDownSelect" :placeholder="''"> </dropdown>
                <div class="text-white mb-7">Amount</div>

                <div class="relative flex items-center">
                  <input ref="depositInput" type="text" placeholder="0" class="w-full absolute block -top-4.5 border sm:text-sm text-gray-50 border-gray-500 text-left bg-gray-800" v-model="selectedToken.maxBalance" />
                  <span class="absolute -top-2.5 right-12 text-blue-400 text-sm" @click="onMax">Max</span>
                  <span class="absolute -top-2.5 right-2 text-gray-400 text-sm">{{ selectedToken.name }}</span>
                </div>
              </div>

              <div class="flex flex-row justify-between">
                <div class="text-white text-xs">Wallet Balance</div>
                <div class="text-white text-xs">{{ format(selectedToken.balance) }} {{ selectedToken.name }}</div>
              </div>

              <div class="mt-10 flex justify-center items-center pb-4">
                <button class="py-2 w-full bg-gray-500 hover:bg-gray-400 text-gray-50" @click="onDeposit">Deposit</button>
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
import TOKENS from '../../config/tokenList'
import dropdown from './DropDown.vue'
import { formatNumber } from '../../utils/token'

export default {
  name: 'DialogDeposit',
  components: {
    TransitionRoot,
    TransitionChild,
    Dialog,
    DialogPanel,
    DialogTitle,
    dropdown,
  },
  computed: {
    ...mapGetters(['isConnected', 'tokens', 'nearConfig', 'currentUser']),
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
    async onDropDownSelect(object) {
      this.selectedToken.name = object.name
      let balance = await this.getBalance(this.selectedToken.name)
      this.selectedToken.balance = balance
      this.selectedToken.maxBalance = ''
    },
    onClose() {
      this.$emit('close')
    },
    async onMax() {
      this.selectedToken.maxBalance = formatNumber(this.selectedToken.balance, 0)
    },
    async onDeposit() {
      let ftContract = this.tokens[this.selectedToken.name]
      console.log('onDeposit', String(this.selectedToken.maxBalance))
      await ftContract.ft_transfer_call(
        {
          receiver_id: this.nearConfig.orderbookContract,
          amount: String(this.selectedToken.maxBalance),
          msg: '',
        },
        this.nearConfig.GAS,
        1
      )
    },
    tokenList() {
      return TOKENS.tokens.map((currElement, index) => {
        return {
          name: currElement.symbol,
          id: index,
        }
      })
    },
    format(val) {
      return formatNumber(val, 4)
    },
    async getBalance(token) {
      let ftContract = this.tokens[token]
      return await ftContract.ft_balance_of({ account_id: this.currentUser.accountId })
    },
  },
}
</script>
