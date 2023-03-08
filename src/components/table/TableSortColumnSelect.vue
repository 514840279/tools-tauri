<template>
    <div id="TableSortColumnSelect">
        <el-select v-model="sortName" size="small" style="width: 100%" @change="handleChange" :filterable="true">
            <el-option v-for="(cl, idx) in sortColumns" :key="idx" :label="cl.sortTitle" :value="cl.sortName"> </el-option>
        </el-select>
    </div>
</template>
<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { SortColumn } from "../../interface/Table";

const parents = withDefaults(
    defineProps<{
        sortColumns: Array<SortColumn>;
        item: SortColumn;
    }>(),
    {
        sortColumns: () => [],
        item: () => {
            return {
                sortIndex: 0,
                sortTitle: "未知",
                sortOrder: "未知",
                sortName: "未知",
            };
        },
    }
);

let sortName = ref<String>();

onMounted(() => {
    sortName.value = parents.item.sortName;
});

function handleChange(va: String) {
    parents.sortColumns.forEach((item) => {
        if (item.sortName == va) {
            parents.item = item;
        }
    });
}
</script>
<style scoped>
#TableSortColumnSelect {
    margin: 0px;
    padding: 0px;
    width: 100%;
}
</style>
