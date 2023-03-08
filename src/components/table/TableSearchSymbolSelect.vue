<template>
    <div id="TableSearchSymbolSelect">
        <el-select v-model="item.symbol" size="small" style="width: 100%" @change="handleChangeSymbol" :filterable="true">
            <el-option v-for="(sy, idx) in symbols" :key="idx" :label="sy.label" :value="sy.value"> </el-option>
        </el-select>
    </div>
</template>
<script lang="ts" setup>
import { SearchParamters } from "../../interface/Table";

const parents = withDefaults(
    defineProps<{
        item?: SearchParamters;
        index?: Number;
        parameters: Array<SearchParamters>;
    }>(),
    {
        item: () => {
            return {
                operator: "eq",
                column: "不知道",
                title: "不知道",
                symbol: "不知道",
                data: "不知道",
                showdata: false,
            };
        },
        index: () => 0,
        parameters: () => [],
    }
);

const symbols = [
    { label: "相等", value: "eq" },
    { label: "不等", value: "notEq" },
    { label: "小于", value: "less" },
    { label: "小于等于", value: "lessAndEq" },
    { label: "大于", value: "great" },
    { label: "大于等于", value: "greatAndEq" },
    { label: "包含", value: "like" },
    { label: "为空", value: "isNull" },
    { label: "不为空", value: "isNotNull" },
];

function handleChangeSymbol(va: String) {
    switch (va) {
        case "eq":
        case "notEq":
        case "less":
        case "lessAndEq":
        case "great":
        case "greatAndEq":
        case "like":
            parents.item.showdata = true;
            break;
        case "isNull":
        case "isNotNull":
        case "isEmpty":
        case "isNotEmpty":
            parents.item.showdata = false;
            break;
        default:
            parents.item.showdata = true;
            break;
    }
}
</script>
<style scoped>
#TableSearchSymbolSelect {
    margin: 0px;
    padding: 0px;
    width: 100%;
}
</style>
