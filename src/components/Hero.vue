<template>
  <div class="flex flex-col text-center mass-header z-10 bg">
    <div class="z-10 flex flex-col md:p-0 p-5 bg">
      <div class="md:mt-48 mt-20">
        <span class="box text-gray-50 font-bold md:text-5xl text-3xl"> </span>
        <span class="text-gray-50 font-bold md:text-5xl text-3xl"> on NEAR</span>
      </div>
      <span class="text-gray-400 text-xl md:mt-10 mt-20"> Guaranteed liquidity trading for millions of users and top NEAR applications <br /> </span>
      <div class="flex justify-center items-start">
        <button class="text-gray-50 md:mt-36 mt-36 px-10 py-5 font-bold bg-gray-800 hover:bg-gray-500 rounded-md" @click="$router.push('/trade')">Trade Now</button>
      </div>
      <div class="flex justify-center text-gray-200 text-xl items-center mt-6">
        <div class="text-left text-[17px] text-gray-500">Built On</div>
        <div class="flex flex-row items-center"><img class="ml-2 h-[1.8rem]" src="https://perp.spin.fi/images/near-wallet-icon.png" alt="logo" /> <span class="ml-3">NEAR</span></div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      isMobile: false,
      obj: {
        output: '',
        isEnd: false,
        speed: 70,
        singleBack: true,
        sleep: 0,
        type: 'normal',
        backSpeed: 70,
        sentencePause: false,
      },
    }
  },
  methods: {
    _isMobile() {
      let flag = navigator.userAgent.match(/(phone|pad|pod|iPhone|iPod|ios|iPad|Android|Mobile|BlackBerry|IEMobile|MQQBrowser|JUC|Fennec|wOSBrowser|BrowserNG|WebOS|Symbian|Windows Phone)/i)
      return flag
    },

    initTyped(obj, input, fn, hooks) {
      const typed = new EasyTyper(obj, input, fn, hooks)
    },
  },

  mounted: function () {
    var typed = new Typed('.box', {
      strings: ['Smart Exchange    '],
      typeSpeed: 100,
      backSpeed: 50,
      backDelay: 800,
      cursorChar: '',
      loop: true,
    })

    this.isMobile = this._isMobile()
  },
}
</script>

<style scoped>
.mass-header {
  width: 100%;
}

.mass-header:before {
  display: block;
  content: '';
  background: rgb(17 24 39);
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  opacity: 0.85;
}
</style>
