<template>
  <TradingViewSimple :market="market" />
</template>

<script>
import TradingViewSimple from './TradingViewSimple.vue'
export default {
  components: { TradingViewSimple },
  data() {},
  props: ['market'],
}
</script>
