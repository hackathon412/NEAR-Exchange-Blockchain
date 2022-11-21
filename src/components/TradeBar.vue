<template>
  <div class="flex sm:flex-row flex-col bg-gray-900 border-black border-b border-t md:py-0 pb-3">
    <div class="md:w-1/4 2xl:w-1/5 w-full mt-0.5">
      <DropDown class="" @selectMarket="updateSelectMarket" />
    </div>
    <div class="w-full">
      <Volume :baseToken="BTC" class="" />
    </div>
  </div>
</template>

<script>
import DropDown from './DropDown.vue'
import Volume from './Volume.vue'
export default {
  components: { DropDown, Volume },
}
</script>

<style></style>
