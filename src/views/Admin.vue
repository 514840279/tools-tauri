<template>
  <el-container>
    <el-header>
      <el-menu :default-active="headMenu.activeIndex" :class="headMenu.class" :mode="headMenu.mode" @select="handleSelect" background-color="#545c64" text-color="#fff" active-text-color="#ffd04b">
        <el-menu-item index="0-2" :style="{ width: asideWidth }">
          <div class="block">
            <el-avatar :size="28" :src="circleUrl" style="vertical-align: middle" />
            <span v-if="!isCollapse"> 后台管理 </span>
          </div>
        </el-menu-item>
        <el-menu-item index="0-1">
          <el-icon :size="56">
            <component :is="ita"></component>
          </el-icon>
        </el-menu-item>
        <el-menu-item v-for="(menu, index) in headMenu.data" :key="index" :index="menu.index">{{ menu.text }} </el-menu-item>
      </el-menu>
    </el-header>
    <el-container>
      <el-aside :width="asideWidth" class="left">
        <el-scrollbar :height="height">

          <el-menu class="el-menu-vertical-demo" :collapse="isCollapse" :default-openeds="openedsIndex" :default-active="aside.activeIndex">
            <el-sub-menu v-for="(subme, index) in aside.submenu" :key="index" :index="subme.index">
              <template #title>
                <el-icon>
                  <!-- <component :is="subme.icon"></component> -->
                </el-icon>
                <span>{{ subme.text }}</span>
              </template>

              <el-menu-item-group v-for="(group, ind) in subme.group" :key="ind" :title="group.title">
                <el-menu-item v-for="(data, i) in group.data" :key="i" :index="data.index">
                  <span>{{ data.text }}</span>
                </el-menu-item>
              </el-menu-item-group>

              <el-menu-item v-for="(data, i) in subme.data" :key="i" :index="data.index">
                <el-icon>
                  <!-- <component :is="data.icon "></component> -->
                </el-icon>
                <span @click="handleBreadcrumb(subme, data)">{{ data.text }}</span>
              </el-menu-item>
            </el-sub-menu>
          </el-menu>
        </el-scrollbar>
      </el-aside>
      <el-container>
        <el-main>
          <el-card class="box-card">
            <template #header>

              <el-breadcrumb v-if="currentList.length > 0" separator="/">
                <el-breadcrumb-item v-for="(item, i) in currentList" :key="i" :index="i" :to="{ path: item.path }">
                  {{ item.text }}
                </el-breadcrumb-item>
              </el-breadcrumb>
            </template>


            <router-view></router-view>
          </el-card>
        </el-main>
        <el-footer class="footer">
          <Foot />
        </el-footer>
      </el-container>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import Foot from "@components/home/Food.vue";
import { onBeforeMount, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { Menu, MenuItem, Aside } from "@interface/Menu";

import { mainStore } from "@/stores/main";
// 使普通数据变响应式的函数
import { storeToRefs } from "pinia";
// 实例化仓库
const store = mainStore();
// 解构并使数据具有响应式
const { headindex, currentIndex, currentPath, currentList, asideWidth, isCollapse, ita } = storeToRefs(store);

let circleUrl = ref<string>("favicon.ico");

// 头部导航菜单
let headMenu = ref<Menu>({
  activeIndex: "0", // 默认页面 index
  class: "el-menu-demo",
  text: "導航",
  mode: "horizontal",
  data: [
    { index: "0", text: "首页" },
    { index: "1", text: "权限管理" },
    { index: "2", text: "数据库管理" },
    { index: "3", text: "分析管理" },
    { index: "4", text: "爬虫管理" },
    { index: "5", text: "消息中心" },
  ],
});
// 替换的菜单
let aside = ref<Aside>({});
// 默认展开menu
const openedsIndex: Array<String> = ["0-1", "1-1", "2-1", "3-1", "4-1", "5-1"];
// 菜单集
let asides = ref<Array<Aside>>([
  {
    activeIndex: "0-1",
    submenu: [
      {
        index: "0-1",
        text: "菜单",
        icon: "Message",
        data: [
          { index: "0-1-1", text: "功能", icon: "Message", link: "" },
          { index: "0-1-2", text: "数据统计", icon: "Message", link: "" },
          { index: "0-1-3", text: "产品设计介绍", icon: "Message", link: "" },
          { index: "0-1-4", text: "其他介绍", icon: "Message", link: "" },
        ],
      },
    ],
  },
  {
    activeIndex: "1-1",
    submenu: [
      {
        index: "1-1",
        text: "权限管理",
        icon: "Message",
        data: [
          { index: "1-1-1", text: "用户管理", link: "" },
          { index: "1-1-2", text: "权限管理", link: "" },
          { index: "1-1-3", text: "项目管理", link: "" },
          { index: "1-1-4", text: "菜单管理", link: "/sysMenuInfo" },
          { index: "1-1-5", text: "消息中心", link: "" },
        ],
      },
    ],
  },
  {
    activeIndex: "2-1",
    submenu: [
      {
        index: "2-1",
        text: "数据管理",
        icon: "Message",
        data: [
          { index: "2-1-2", text: "数据类型", link: "/databasetype" },
          { index: "2-1-4", text: "索引类型", link: "/index" },
          { index: "2-1-1", text: "数据微服务", link: "/database" },
          { index: "2-1-3", text: "表管配置理", link: "/tables" },
          { index: "2-1-5", text: "字段管理", link: "/columns" },
          { index: "2-1-6", text: "表查询", link: "/tabsSearch" },
          { index: "2-1-7", text: "表数据查询", link: "/searchData" },
          { index: "2-1-8", text: "类型查询", link: "/searchIndex" },
          { index: "2-1-9", text: "类型数据查询", link: "/searchIndexData" },
        ],
      },

    ],
  },
  {
    activeIndex: "3-1",
    submenu: [
      {
        index: "3-1",
        text: "分析管理",
        icon: "Message",
        data: [
          { index: "3-1-1", text: "模板管理", link: "" },
          { index: "3-1-2", text: "图形搭配", link: "" },
        ],
      },
    ],
  },
  {
    activeIndex: "4-1",
    submenu: [
      {
        index: "4-1",
        text: "爬虫管理",
        icon: "Message",
        data: [
          { index: "4-1-1", text: "机器信息", link: "" },
          { index: "4-1-2", text: "爬虫项目信息", link: "" },
          { index: "4-1-3", text: "爬虫项目管理", link: "" },
        ],
      },
    ],
  },
  {
    activeIndex: "5-1",
    submenu: [
      {
        index: "5-1",
        text: "消息中心",
        icon: "Message",
        data: [
          { index: "5-1-1", text: "消息中心", link: "" },
          { index: "5-1-2", text: "消息统计分析", link: "" },
          { index: "5-1-2", text: "网站链接", link: "" },
        ],
      },
    ],
  },
]);
let height = ref<string>("680px");

let router = useRouter();
onBeforeMount(() => {
  const h = document.documentElement.clientHeight;
  height.value = h - 59 + "px";
});

onMounted(() => {
  //  根据不同权限 headMenu，asides 应该设置不同的数据，
  init();
});

// 初始化展示信息
function init(): void {
  let index = null;
  // pinia 用户使用记录到本地
  if (currentPath.value != "") {
    headMenu.value.activeIndex = headindex.value;
    aside.value = asides.value[Number(headindex.value)];
    aside.value.activeIndex = currentIndex.value;
    // 更换默认页面
    router.push(currentPath.value);
  } else {
    // TODO user set

    // default set
    index = headMenu.value.activeIndex;
    if (typeof index == "string") {
      handleSelect(index);
    }
  }
}

// head 头部点击事件 切换左侧导航信息 ，更换路由
function handleSelect(index: string): void {
  if (index == "0-2") {
    handleSelect("0");
  } else if (index == "0-1") {
    if (!isCollapse.value == false) {
      isCollapse.value = !isCollapse.value;
      asideWidth.value = "200px";
      ita.value = "Fold";
    } else {
      isCollapse.value = !isCollapse.value;
      asideWidth.value = "66px";
      ita.value = "Menu";
    }
  } else {
    headindex.value = String(index);
    // 切换aside
    aside.value = asides.value[Number(index)];
    let activeIndex = aside.value.activeIndex;

    let submenus = aside.value.submenu;

    // default set
    let path = "/home" + index;

    currentList.value = [];
    currentList.value[0] = { path: path, text: headMenu.value.data[Number(index)].text };

    currentPath.value = path;

    // 根据 activeIndex 和 submenu.index 确定默认展示页面 是 home${index} 还是data.link
    submenus?.forEach((submenu, inex) => {
      if (submenu.index == activeIndex) {
        currentIndex.value = String(activeIndex);
        // 更换默认页面
        router.push(path);
      } else if (String(activeIndex).indexOf(String(submenu.index)) > -1) {
        submenu.data?.forEach((data) => {
          if (data.index == activeIndex) {
            handleBreadcrumb(submenu, data);
          }
        });
      }
    });
  }
}

// aside 左侧点击事件切换面包屑信息
function handleBreadcrumb(submenu: Menu | MenuItem, data: MenuItem): void {
  currentList.value[1] = { text: submenu.text };
  currentList.value[2] = { text: data.text };
  currentIndex.value = String(data.index);
  var path = data.link;
  currentPath.value = String(path);
  if (typeof path == "string") {
    // 更换默认页面
    router.push(path);
  }
}
</script>

<style>
html,
body {
  margin: 0px;
  padding: 0px;
  min-height: 500px;
}

.el-header {
  padding: 0px;
}

.memu-card {
  padding: 0px;
}

.left {
  min-height: 800px;
}

.footer {
  height: 35px;
  background-color: rgba(247, 245, 230, 0.733);
  text-align: center;
  padding: 8px 0px 5px 0px;
}
</style> 