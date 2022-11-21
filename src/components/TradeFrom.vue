<template>
  <div class="flex flex-col px-0.5 h-full">
    <div class="bg-gray-900 h-full">
      <div class="p-2 bg-gray-900 h-full">
        <ul class="flex rounded-lg divide-gray-200 shadow sm:flex">
          <li class="w-full">
            <button class="inline-block relative py-2 px-4 w-full text-sm font-medium text-center text-gray-50 bg-gray-800 hover:text-gray-700 hover:bg-gray-600 focus:bg-green-300" @click="toggleTabs('buy')" v-bind:class="{ 'bg-green-300': tab === 'buy' }">BUY</button>
          </li>
          <li class="w-full">
            <button class="inline-block relative py-2 px-4 w-full text-sm font-medium text-center text-gray-50 bg-gray-800 hover:text-gray-700 hover:bg-gray-600 focus:bg-red-400" @click="toggleTabs('sell')" v-bind:class="{ 'bg-red-400': tab === 'sell' }">SELL</button>
          </li>
        </ul>

        <!-- order type -->
        <div class="pt-2 text-sm font-medium text-center text-gray-500 border-b border-gray-600">
          <ul class="flex flex-wrap -mb-px">
            <li class="mr-2">
              <a href="javascript:void(0)" class="inline-block p-2 rounded-t-lg border-b-2 border-transparent hover:text-gray-300 hover:border-blue-600" @click="tabOrderType('market')" v-bind:class="{ 'border-blue-600 text-gray-300': orderType === 'market' }">Market</a>
            </li>
            <li class="mr-2">
              <a href="javascript:void(0)" class="inline-block p-2 rounded-t-lg border-b-2 border-transparent hover:text-gray-300 hover:border-blue-600" @click="tabOrderType('limit')" v-bind:class="{ 'border-blue-600 text-gray-300': orderType === 'limit' }">Limit</a>
            </li>
          </ul>
        </div>
        <!-- order type -->

        <!-- market order tab  -->
        <div v-if="orderType === 'market'">
          <div class="mt-3 flex items-center rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500">
            <span class="w-3/12 text-gray-200 border border-gray-600 py-2 sm:text-sm rounded-l text-center"> Price </span>
            <div class="relative w-9/12 flex items-center">
              <input type="text" class="w-full absolute block pr-12 -top-4.5 border sm:text-sm text-gray-50 border-gray-500 text-center bg-gray-800" disabled placeholder="Market Price" />
              <div class="absolute -top-2.5 right-2 text-gray-400 text-sm">{{ quoteToken.name }}</div>
            </div>
          </div>

          <div class="mt-3 flex items-center rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500">
            <span class="w-3/12 text-gray-200 border border-gray-600 py-2 sm:text-sm rounded-l text-center"> Size </span>
            <div class="relative w-9/12 flex items-center">
              <input type="number" class="w-full absolute block pr-12 -top-4.5 border sm:text-sm text-gray-50 border-gray-500 text-center bg-gray-800" step="0.001" v-model="amount" :min="0" oninput="if(value<0)value=0" />
              <div class="absolute -top-2.5 right-2 text-gray-400 text-sm">{{ tab === 'sell' && orderType === 'market' ? baseToken.name : quoteToken.name }}</div>
            </div>
          </div>
        </div>
        <!-- market order tab  -->

        <!-- limit order tab  -->
        <div v-if="orderType === 'limit'">
          <div class="mt-3 flex items-center rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500">
            <span class="w-3/12 text-gray-200 border border-gray-600 py-2 sm:text-sm rounded-l text-center"> Price </span>
            <div class="relative w-9/12 flex items-center">
              <input type="number" class="w-full absolute block pr-12 -top-4.5 border sm:text-sm text-gray-50 border-gray-500 text-center bg-gray-800" step="0.001" v-model="price" :min="0" oninput="if(value<0)value=0" />
              <div class="absolute -top-2.5 right-2 text-gray-400 text-sm">{{ quoteToken.name }}</div>
            </div>
          </div>
          <div class="mt-3 flex items-center rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500">
            <span class="w-3/12 text-gray-200 border border-gray-600 py-2 sm:text-sm rounded-l text-center"> Size </span>
            <div class="relative w-9/12 flex items-center">
              <input type="number" class="w-full absolute block pr-12 -top-4.5 border sm:text-sm text-gray-50 border-gray-500 text-center bg-gray-800" step="0.001" v-model="amount" :min="0" oninput="if(value<0)value=0" />
              <div class="absolute -top-2.5 right-2 text-gray-400 text-sm">{{ baseToken.name }}</div>
            </div>
          </div>
        </div>
        <!-- limit order tab  -->

        <div class="flex justify-center mt-3 mb-2">
          <RangeSlider @sliderPercentUpdate="updatePercent" />
        </div>

        <div class="mt-2 flex justify-center items-center">
          <button class="py-2 w-full bg-gray-800 hover:bg-gray-600 text-gray-50" @click="onTrade">{{ opBtnTxt }}</button>
        </div>
        <Assets />
      </div>
    </div>
  </div>
</template>

<script>
export default {}
</script>

<style></style>
