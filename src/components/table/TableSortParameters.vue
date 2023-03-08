<template>
    <div id="TableSortParameters">
        <el-row v-for="(item, index) in parents.sortParameters" :key="index" class="prow">
            <el-col :span="1">
                <el-button v-if="index > 0" @click="handleTopSort(item, index)" icon="CaretTop" circle size="small" title="添加"></el-button>
            </el-col>
            <el-col :span="6">
                <TableSortColumnSelect :sortColumns="sortColumns" :item="item" :index="index" v-model:parameters="parents.sortParameters"></TableSortColumnSelect>
            </el-col>
            <el-col :span="1">
                <el-button @click="handleSortOrder(item, index)" size="small">{{ item.sortOrder }}</el-button>
            </el-col>
            <el-col :span="1">
                <el-button @click="handleDelSort(item, index)" icon="Remove" circle size="small" title="删除"></el-button>
            </el-col>
        </el-row>
        <el-row>
            <el-col :span="8">
                <el-button @click="handleAddSort" type="success" icon="Plus" circle size="small" title="添加"></el-button>
                <el-button @click="handleSort" type="success" icon="Check" circle size="small" title="确定"></el-button>
            </el-col>
        </el-row>
    </div>
</template>
<script lang="ts" setup>
import TableSortColumnSelect from "./TableSortColumnSelect.vue";
import { onMounted, ref, getCurrentInstance } from "vue";
import { SortColumn } from "../../interface/Table";

const parents = withDefaults(
    defineProps<{
        sortColumns: Array<SortColumn>;
        sortParameters: Array<SortColumn>;
    }>(),
    {
        sortColumns: () => [],
        item: () => [],
    }
);
let columns = ref<Array<SortColumn>>([]),
    orderSize = ref<number>(0),
    plusNum = ref<number>(0),
    columnsSize = ref<number>(0);
const Message = getCurrentInstance();
const emit = defineEmits(["sortTable"]);

onMounted(() => {
    columns.value = parents.sortColumns;
    orderSize.value = 0;
    plusNum.value = 0;
    columnsSize.value = 0;
});

// 添加新的排序条件
function handleAddSort() {
    // 防阻size为0
    if (columnsSize.value == 0) {
        columnsSize.value = parents.sortColumns.length;
    }
    // 保证循环添加默认值数据
    if (columnsSize.value == plusNum.value) {
        // TODO 提示消息 “没有课排序的字段了！！”
        return;
    }
    // 添加新的排序条件
    // parameters.value.push(columns.value[plusNum.value]);

    for (let index = 0; index < parents.sortColumns.length; index++) {
        const i1 = parents.sortColumns[index];
        let a: Boolean = true;
        parents.sortParameters.forEach((item) => {
            if (item.sortName === i1.sortName) {
                a = false;
            }
        });
        if (a) {
            parents.sortParameters.push(i1);
            // 计算数量的
            orderSize.value++;
            // 循环下标控制
            plusNum.value++;
            handleSortIndex();
            return;
        }
    }
}

// 删除排序条件
function handleDelSort(item: SortColumn, index: number) {
    parents.sortParameters.splice(index, 1);
    orderSize.value--;
    handleSortIndex();
}

// 条件前移
function handleTopSort(item: SortColumn, index: number) {
    parents.sortParameters.splice(index, 1);
    parents.sortParameters.splice(index - 1, 0, item);
    handleSortIndex();
}

// 排序
function handleSortIndex() {
    let a = 1;
    parents.sortParameters.forEach((item) => {
        item.sortIndex = a;
        parents.sortParameters.splice(a - 1, 1, item);
        a++;
    });
}

// 取排序
function handleSort() {
    emit("sortTable");
}

// 排序顺序控制
function handleSortOrder(item: SortColumn, index: number) {
    if (item.sortOrder == "asc") {
        item.sortOrder = "desc";
    } else {
        item.sortOrder = "asc";
    }
}
</script>
<style lang="scss" scoped>
#TableSortParameters {
    margin: 0px;
    padding: 8px;
    background: rgb(248, 248, 248);
    min-height: 150px;
    max-height: 320px;
    overflow-y: auto;
}
</style>
