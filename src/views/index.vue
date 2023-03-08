<template>
    <div class="common-layout">
        <el-container>
            <el-header>
                <el-menu :default-active="activeIndex" class="el-menu-demo" mode="horizontal" :ellipsis="false" @select="handleSelect">
                    <el-menu-item index="0">LOGO</el-menu-item>
                    <div class="flex-grow" />
                    <div v-for="(subme, index) in headMenu.submenu" :key="index">
                        <el-menu-item v-if="subme.data == null">
                            <el-icon>
                                <component :is="subme.icon"></component>
                            </el-icon>
                            <span @click="handleBreadcrumb(subme)">{{ subme.text }}</span>
                        </el-menu-item>
                        <el-sub-menu v-else :style="{ width: subme.width }">
                            <template #title>
                                <el-icon>
                                    <component :is="subme.icon"></component>
                                </el-icon>
                                <span>{{ subme.text }}</span>
                            </template>
                            <!-- submenu -->
                            <el-menu-item v-for="(data, i) in subme.data" :key="i" :index="data.index">
                                <el-icon>
                                    <component :is="data.icon"></component>
                                </el-icon>
                                <span @click="handleBreadcrumb(data)">{{ data.text }}</span>
                            </el-menu-item>
                        </el-sub-menu>
                    </div>
                </el-menu>
            </el-header>
            <el-main>Main</el-main>
        </el-container>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Menu, MenuItem, Aside } from "@interface/Menu";

const activeIndex = ref("1");
let headMenu: Aside = {
    activeIndex: "1",
    submenu: [
        {
            index: "1",
            text: "首页",
            icon: "Message",
            link: "",
            width: "130",
        },
        {
            index: "2",
            text: "消息中心",
            icon: "Message",
            width: "140px",
            data: [
                { index: "2-1", text: "消息中心", icon: "Message", link: "" },
                { index: "2-2", text: "消息中心2", icon: "Message", link: "" },
            ],
        },
    ],
};

const handleSelect = (key: string, keyPath: string[]) => {
    console.log(key, keyPath);
};

function handleBreadcrumb(data: Menu | MenuItem) { }
</script>

<style scoped>
.flex-grow {
    flex-grow: 1;
}
</style>
