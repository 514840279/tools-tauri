<template>
    <div id="TableColumnSelect">
        <el-popover placement="bottom" title="列筛选" trigger="click">
            <template #reference>
                <el-button icon="CaretBottom" circle size="small" title="列选择"></el-button>
            </template>
            <el-checkbox-group v-model="checkList" @change="filter" style="max-width: 1500px">
                <ul class="infinite-list" style="overflow: auto">
                    <li v-for="(item, index) in tableList" :key="index" class="infinite-list-item">
                        <el-checkbox v-model="item.title" :label="item.title" size="small"></el-checkbox>
                    </li>
                </ul>
            </el-checkbox-group>
        </el-popover>
    </div>
</template>
<script lang="ts" setup>
import { onBeforeMount, ref } from "vue";
import { Column } from "../../interface/Table";

const parents = withDefaults(
    defineProps<{
        columns?: Array<Column>;
    }>(),
    {
        columns: () => [],
    }
);

let tableList = ref<Array<Column>>([]),
    checkList = ref<Array<any>>([]),
    list = ref<Array<any>>([]),
    result: Array<any> = [];

const emit = defineEmits(["update:columns"]);

onBeforeMount(() => {
    tableList.value = parents.columns;

    tableList.value.forEach((item, index) => {
        if (item.show == null) {
            item.show = true;
        }
        if (item.show) {
            checkList.value.push(item.title);
        }
        list.value.push(item.title);
    });
});

function filter(val: Array<String>) {
    result = list.value.filter((number) => !val.includes(number));

    if (result.length >= 1) {
        result.forEach((item, index) => {
            tableList.value.forEach((items, indexs) => {
                if (items.title == item) {
                    items.show = false;
                }
            });
        });
    }
    val.forEach((item, index) => {
        tableList.value.forEach((items, indexs) => {
            if (items.title == item) {
                items.show = true;
            }
        });
    });
    emit("update:columns", tableList.value);
}
</script>
<style scoped lang="scss">
#TableColumnSelect {
    font-size: 20px;
    padding: 0px;
    margin: 0px;
    margin-left: 10px;
    white-space: nowrap;
    display: inline-block;
    min-height: 40px;
}

.infinite-list {
    height: 180px;
    padding: 0;
    margin: 0;
    list-style: none;

    .infinite-list-item {
        display: flex;
        justify-content: left;
        height: 20px;
        background: var(--el-color-primary-light-9);
        margin: 2px;
        color: var(--el-color-primary);

        &+.list-item {
            margin-top: 10px;
        }
    }
}
</style>
