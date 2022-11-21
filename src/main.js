import { createApp } from 'vue';
import App from './App.vue';
import './assets/tailwind.css';
import router from './router';
import store from './store';
import FloatingVue from 'floating-vue'
import 'floating-vue/dist/style.css'
import 'mosha-vue-toastify/dist/style.css'

createApp(App)
  .use(FloatingVue)
  .use(store)
  .use(router)
  .mount(document.body)
