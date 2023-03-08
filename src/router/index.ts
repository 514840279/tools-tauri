import { createRouter, createWebHistory } from "vue-router";

// import Index from '../views/index.vue'
// import Admin from '../views/Admin.vue'
import Home0 from '../views/home/Home0.vue'
import Home1 from '../views/home/Home1.vue'
import Home2 from '../views/home/Home2.vue'
import Home3 from '../views/home/Home3.vue'
import Home4 from '../views/home/Home4.vue'
import Home5 from '../views/home/Home5.vue'
import Home6 from '../views/home/Home6.vue'

import Err from '../views/Error.vue'



// 2. 定义一些路由
// 每个路由都需要映射到一个组件。
// 我们后面再讨论嵌套路由。
const routes = [
  {
  //   path: '/',
  //   redirect: '/index'   //重定向
  // }, {
  //   path: '/index',
  //   name: "index",
  //   component: Index,
  // }, {
  //   path: '/admin',
  //   name: "admin",
  //   component: Admin,
  // }, {
    path: '/home0',
    name: "home0",
    component: Home0,
  }, {
    path: '/home1',
    name: "home1",
    component: Home1,
  }, {
    path: '/home2',
    name: "home2",
    component: Home2,
  }, {
    path: '/home3',
    name: "home3",
    component: Home3,
  }, {
    path: '/home4',
    name: "home4",
    component: Home4,
  }, {
    path: '/home5',
    name: "home5",
    component: Home5,
  }, {
    path: '/home6',
    name: "home6",
    component: Home6,
  }, {
    path: '/home7',
    name: "home7",
    component: () => import('../views/home/Home7.vue'),
  }, {
    path: '/databasetype',
    name: "databasetype",
    component: () => import('../views/database/type/Index.vue'),
  },{
    path: '/error',
    name: "error",
    component: Err,
  },{
    path: '/:catchAll(.*)',
    redirect: '/error'   //重定向
  },
]

// 3. 创建路由实例并传递 `routes` 配置
// 你可以在这里输入更多的配置，但我们在这里
// 暂时保持简单
export const router = createRouter({
  // 4. 内部提供了 history 模式的实现。为了简单起见，我们在这里使用 hash 模式。
  // history: createWebHashHistory(),
  history: createWebHistory(),
  routes, // `routes: routes` 的缩写
})
