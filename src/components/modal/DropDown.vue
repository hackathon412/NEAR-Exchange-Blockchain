<template>
  <div class="btn-group w-full">
    <li @click="toggleMenu()" class="p-2 flex flex-row items-center text-gray-500 hover:bg-gray-600" v-if="selectedOption.name !== undefined">
      <img class="h-6 mr-2" :src="getTokenLogo(selectedOption.name)" alt="" />
      {{ selectedOption.name }}
      <span class="caret"></span>
    </li>

    <li @click="toggleMenu()" class="dropdown-toggle dropdown-toggle-placeholder" v-if="selectedOption.name === undefined">
      {{ placeholderText }}
      <span class="caret"></span>
    </li>

    <ul class="dropdown-menu bg-gray-800" v-if="showMenu">
      <div v-for="(option, idx) in options" :key="idx" class="text-gray-50">
        <a href="javascript:void(0)" class="flex py-2 px-2 hover:bg-gray-500 justify-between items-center" @click="updateOption(option)">
          <div class="flex flex-row justify-center items-center">
            <img class="h-6 mr-2" :src="getTokenLogo(option.name)" alt="" />
            <span>{{ option.name }}</span>
          </div>
          <div>
            <span>{{ getTokenBalance(option.name) }}</span>
          </div>
        </a>
      </div>
    </ul>
  </div>
</template>

<script>
import { getTokenAddress, getTokenCfg, formatNumber } from '../../utils/token'
import { mapGetters } from 'vuex'

export default {
  data() {
    return {
      selectedOption: {
        name: '',
      },
      showMenu: false,
      placeholderText: 'Please select an item',
      balances: [],
    }
  },
  props: {
    options: {
      type: [Array, Object],
    },
    selected: {},
    placeholder: [String],
    closeOnOutsideClick: {
      type: [Boolean],
      default: true,
    },
  },
  computed: {
    ...mapGetters(['isConnected', 'account', 'assetBalance', 'tokens', 'currentUser']),
  },
  mounted() {
    this.selectedOption = this.selected
    if (this.placeholder) {
      this.placeholderText = this.placeholder
    }
    if (this.closeOnOutsideClick) {
      document.addEventListener('click', this.clickHandler)
    }
  },
  beforeDestroy() {
    document.removeEventListener('click', this.clickHandler)
  },
  methods: {
    updateOption(option) {
      this.selectedOption = option
      this.showMenu = false
      this.$emit('updateOption', this.selectedOption)
    },
    toggleMenu() {
      this.showMenu = !this.showMenu
      if (this.showMenu) {
        this.balances = []
        Promise.all(this.options.map(async (item) => (this.balances[item.name] = formatNumber(await this.tokens[item.name].ft_balance_of({ account_id: this.currentUser.accountId })))))
      }
    },
    clickHandler(event) {
      const { target } = event
      const { $el } = this
      if (!$el.contains(target)) {
        this.showMenu = false
      }
    },
    getTokenLogo(symbol) {
      return getTokenCfg(symbol)?.logoURI
    },
    getTokenBalance(symbol) {
      return this.balances[symbol]
    },
  },
}
</script>

<style>
.btn-group {
  min-width: 100%;
  height: 40px;
  position: relative;
  margin: 10px 0px;
  display: inline-block;
  vertical-align: middle;
}
.btn-group a:hover {
  text-decoration: none;
}

.dropdown-toggle:hover {
  background-color: rgb(27, 29, 28);
}

.dropdown-menu {
  z-index: 1000;
}

.dropdown-menu > li {
  /* padding:0 20px; */
  overflow: hidden;
  width: 100%;
  position: relative;
  margin: 0;
}

.caret {
  width: 0;
  position: absolute;
  top: 19px;
  height: 0;
  margin-left: -24px;
  vertical-align: middle;
  border-top: 4px dashed;
  border-top: 4px solid \9;
  border-right: 4px solid transparent;
  border-left: 4px solid transparent;
  right: 10px;
}

li {
  list-style: none;
}
</style>
