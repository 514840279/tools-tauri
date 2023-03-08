<template>
    <div id="TableSearchColumnSelect">
        <el-select v-model="value" size="small" style="width: 100%" @change="handleChange" :filterable="true">
            <el-option v-for="(cl, idx) in columns" :key="idx" :label="cl.searchTitle" :value="cl.searchName"> </el-option>
        </el-select>
    </div>
</template>
<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { SearchColumn, SearchParamters } from "../../interface/Table";

const parents = withDefaults(
    defineProps<{
        columns: Array<SearchColumn>;
        item: SearchParamters;
        index: Number;
        parameters: Array<SearchParamters>;
    }>(),
    {
        columns: () => [],
        item: () => {
            return {
                operator: "eq",
                column: "不知道",
                title: "不知道",
                symbol: "不知道",
                data: "不知道",
                showdata: false,
                searchType: "",
            };
        },
        index: () => 0,
        parameters: () => [],
    }
);

let value = ref<String>();

onMounted(() => {
    value.value = parents.item.column;
});

function handleChange(va: String) {
    parents.columns.forEach((item) => {
        if (item.searchName == va) {
            parents.item.column = item.searchName;
            parents.item.title = item.searchTitle;
            parents.item.searchPlaceholder = item.searchPlaceholder;
        }
    });
}
</script>
<style scoped>
#TableSearchColumnSelect {
    margin: 0px;
    padding: 0px;
    width: 100%;
}
</style>
