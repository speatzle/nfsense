import './global-styles/atomic.css';
import './global-styles/components.css';
import './global-styles/colors.css';
import './global-styles/mlfe.css';
import './global-styles/transitions.css';
import 'vue-toast-notification/dist/theme-default.css';

import PillBar from "./components/inputs/PillBar.vue";
import TextBox from "./components/inputs/TextBox.vue";
import NumberBox from "./components/inputs/NumberBox.vue";
import MultilineTextBox from "./components/inputs/MultilineTextBox.vue";
import CheckBox from "./components/inputs/CheckBox.vue";
import SingleSelect from './components/inputs/SingleSelect.vue';
import MultiSelect from './components/inputs/MultiSelect.vue';

import { Form, Field, ErrorMessage  } from 'vee-validate';

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

// Global Components
app.component('PillBar', PillBar);
app.component('TextBox', TextBox);
app.component('NumberBox', NumberBox);
app.component('MultilineTextBox', MultilineTextBox);
app.component('CheckBox', CheckBox);
app.component('ValidationForm', Form);
app.component('Field', Field);
app.component('ErrorMessage', ErrorMessage);
app.component('SingleSelect', SingleSelect);
app.component('MultiSelect', MultiSelect);

app.mount('#app');
