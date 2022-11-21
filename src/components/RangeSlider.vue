<template>
  <div class="slider" @click="barClick">
    <div class="">
      <div class="relative">
        <input style="z-index: -1; width: 100%;" type="range" value="0" class="mb-4 h-1 bg-gray-600 rounded-lg appearance-none cursor-pointer range-sm" id="slider" @mouseup="emitPercent" @click="updatePercent" @mouseenter="showPercent = true" @mousemove="updatePercent($refs.value)" ref="input" @mouseleave="showPercent = false" />
        <div class="w-full" v-if="showPercent">
          <div ref="sel" id="selectBtn" class="-top-3 h-4 w-8 text-center rounded-sm bg-transparent absolute text-blue-600 text-xs m-auto cursor-none">{{ percent }}%</div>
        </div>

        <span style="left: 25%; top: 35%; z-index: -10;" class="h-2 w-2 bg-gray-600 rounded-lg absolute"></span>
        <span style="left: 50%; top: 35%; z-index: -10;" class="h-2 w-2 bg-gray-600 rounded-lg absolute"></span>
        <span style="left: 74%; top: 35%; z-index: -10;" class="h-2 w-2 bg-gray-600 rounded-lg absolute"></span>
        <span style="right: 0.3px; top: 35%; z-index: -10;" class="h-2 w-2 bg-gray-600 rounded-lg absolute"></span>
      </div>
    </div>
    <div class="w-full flex justify-between text-xs">
      <span class="text-white">0%</span>
      <span class="text-white">25%</span>
      <span class="text-white">50%</span>
      <span class="text-white">75%</span>
      <span class="text-white">100%</span>
    </div>
  </div>
</template>

<script>
export default {
  data: function () {
    return {
      percent: 0,
      showPercent: false,
    }
  },
  mounted() {},
  methods: {
    updatePercent() {
      this.percent = this.$refs.input.value
      this.$refs.sel.style.left = this.$refs.input.value - 2 + '%'
      if (this.percent > 50) {
        this.$refs.sel.style.left = this.$refs.input.value - 7 + '%'
      }
    },

    emitPercent() {
      this.$emit('sliderPercentUpdate', this.percent)
    },
  },
}
</script>

<style scoped>
.slider {
  z-index: 1;
  width: 100%;
  padding: 10px 0;
}
</style>
