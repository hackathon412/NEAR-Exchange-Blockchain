import {
  createRouter,
  createWebHistory
} from 'vue-router';
import Home from '../views/Home.vue';
import Trade from '../views/Trade.vue';
import Faucet from '../views/Faucet.vue';

const routes = [{
  path: '/',
  name: 'Home',
  component: Home
},
{
  path: '/trade',
  name: 'trade',
  component: Trade
},
{
  path: '/faucet',
  name: 'faucet',
  component: Faucet
},
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
