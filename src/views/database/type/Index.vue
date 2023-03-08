<template>
    <div>
        <Table :columns="columns" :rootUrl="rootUrl" />
    </div>
</template>

<script setup lang="ts">
import Table from '../../../components/table/Table.vue'
import { Column } from '../../../interface/Table'

let rootUrl: String = '/serve/sysDbmsTabsTypeInfo',
    columns: Array<Column> = [{
        name: "uuid",
        title: "uuid",
        align: 'left',
        show: false,
    }, {
        name: "typeName",
        title: "类型名",
        align: 'left',
        sort: true,
        search: true
    }, {
        name: "typeIcon",
        title: "icon",
        align: 'center',
        sort: true,
        search: true
    }, {
        name: "typeClass",
        title: "class",
        align: 'left',
        sort: true,
        search: true
    }, {
        name: "typeOrder",
        title: "显示顺序",
        align: 'left',
        sort: true,
        // search: true
    }];


</script>

<style scoped>
</style>