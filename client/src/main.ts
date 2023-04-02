import './global-styles/atomic.css';
import './global-styles/components.css';
import './global-styles/colors.css';
import './global-styles/mlfe.css';
import './global-styles/transitions.css';
import 'vue-toast-notification/dist/theme-default.css';

import App from './App.vue';
import { createHead } from '@vueuse/head';
import { createRouter, createWebHistory } from 'vue-router';
import routes from '~pages';
import ToastPlugin from 'vue-toast-notification';

const app = createApp(App);
const head = createHead();
const router = createRouter({
  history: createWebHistory(),
  routes,
});

app.use(router);
app.use(head);
app.use(ToastPlugin);

app.mount('#app');
