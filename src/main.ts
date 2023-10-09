import { createApp } from "vue";

import App from "./App.vue";
// 使用naive ui
import naive from 'naive-ui';
// 通用字体
import 'vfonts/Lato.css';
// 等宽字体
import 'vfonts/FiraCode.css';

import axios from 'axios';
import VueAxios from 'vue-axios';
import { createPinia } from 'pinia';
// 持久化存储pinia
import piniaPluginPersist from 'pinia-plugin-persist'
import { router } from './router'


const pinia = createPinia()
pinia.use(piniaPluginPersist)

// 自定义的样式
import "./styles.css";
const app = createApp(App)

app.use(VueAxios, axios);
app.use(pinia);
app.use(naive);
app.use(router);
app.mount("#app");