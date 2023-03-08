import { defineStore } from 'pinia'
import { Breadcrumb } from "@interface/Menu";


let currentListData: Array<Breadcrumb> = [];


// useStore could be anything like useUser, useCart
// the first argument is a unique id of the store across your application
export const mainStore = defineStore('main', {
  // other options...
  state: () => {
    return {
      headindex: "",
      currentIndex: "",
      currentPath: "",
      currentList: currentListData, // 面包屑
      asideWidth: "200px",
      isCollapse: false,
      ita: "Fold",
    }
  },
  persist: {
    enabled: true,
    // 自定义持久化参数
    strategies: [
      {
        // 自定义key
        key: 'main_store',
        // 自定义存储方式，默认sessionStorage
        storage: localStorage,
        // 指定要持久化的数据，默认所有 state 都会进行缓存，可以通过 paths 指定要持久化的字段，其他的则不会进行持久化。
        paths: ['headindex', 'currentIndex', "currentPath", "currentList", "asideWidth", "isCollapse", "ita"],
      }
    ]
  },
  getters: {},
  actions: {}
})