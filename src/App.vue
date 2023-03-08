<script setup lang="ts">
import { ref } from "vue";

import { darkTheme, lightTheme } from 'naive-ui';
import { zhCN, dateZhCN } from 'naive-ui';
import { NConfigProvider } from 'naive-ui';

import Admin from "./views/Admin.vue";
import Index from "./views/index.vue";
import Login from "./views/Login.vue";
import Error from "./views/Error.vue";

import { ViewType } from "./interface/View";

import { userStore } from "@/stores/user";

// 实例化仓库
const store = userStore();
// 获取数据
const userinfo = store.userinfo;

let viewF = ref<ViewType>(ViewType.Login);
if (userinfo == null) {
  viewF.value = ViewType.Login;
} else {
  // 判断登录后信息展示 默认登录页面
  if (userinfo.roles?.includes("admin")) {
    viewF.value = ViewType.Admin;
  } else if (userinfo.roles?.includes("user")) {
    viewF.value = ViewType.User;
  } else {
    viewF.value = ViewType.Login;
  }
}
</script>

<template>
  <!-- 调整 naive-ui 的字重配置 -->
  <n-config-provider :locale="zhCN" :date-locale="dateZhCN" :theme="darkTheme">
    <!-- <keep-alive> -->
    <Admin v-if="viewF === ViewType.Admin" />
    <Index v-if="viewF === ViewType.User" />
    <Login v-if="viewF === ViewType.Login" />
    <Error v-if="viewF === ViewType.Error" />
  </n-config-provider>
</template>

<style scoped></style>
