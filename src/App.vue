<template>
  <div class="flex flex-col h-screen">
    <Navbar />
    <div class="flex-1">
      <router-view />
    </div>
  </div>
</template>

<script>
import Navbar from "./components/Navbar.vue"
export default {
  components: {
    Navbar,
  },
}
</script>
