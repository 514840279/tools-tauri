<template>
    <div id="TableSave">
        <el-form :label-position="parents.labelPosition" :model="row" label-width="120px" ref="form">
            <el-form-item :key="index" v-for="(item, index) in parents.columns">
                <template v-slot:label>
                    {{ item.title }}<span style="color: red; margin-left: 5px">{{ item.subtitle }}</span>
                </template>
                <Icon v-model:icon="row[item.name]" type="info" v-if="item.searchType == SearchType.ICON"></Icon>
                <el-select :placeholder="item.name" size="small" v-else-if="item.searchType == SearchType.SELECT" v-model="row[item.name]" :disabled="item.disable">
                    <el-option :key="op.value" :label="op.label" :value="op.value" v-for="op in item.searchDataArray" />
                </el-select>
                <el-input-number v-else-if="item.searchType == SearchType.INTEGER" v-model="row[item.name]" :disabled="item.disable"></el-input-number>
                <el-radio-group v-else-if="item.searchType == SearchType.REDIO" v-model="row[item.name]" :disabled="item.disable">
                    <el-radio :key="op.value" :label="op.value" v-for="op in item.searchDataArray">{{ op.label }}</el-radio>
                </el-radio-group>
                <el-color-picker v-else-if="item.searchType == SearchType.COLOR" v-model="row[item.name]" />
                <el-input v-else v-model="row[item.name]" :disabled="item.disable"></el-input>
            </el-form-item>
            <el-form-item>
                <el-button @click="onSave" type="primary">保存</el-button>
                <el-button @click="onCancel">取消</el-button>
            </el-form-item>
        </el-form>
    </div>
</template>
<script lang="ts" setup>
import { onBeforeMount, ref, watch } from "vue";
import { Column, SearchType, SearchParamters } from "../../interface/Table";
import Icon from "../select/icon/Index.vue";

const parents = withDefaults(
    defineProps<{
        info?: any;
        columns: Array<Column>;
        labelPosition?: String;
    }>(),
    {
        info: () => { },
        columns: () => [],
        labelPosition: () => "top",
    }
);

const emit = defineEmits(["onSave", "update:info", "onCancel", "searchTable"]);

let localSearchParameters = ref<Array<SearchParamters>>([]);

let row = ref<any>({});
let localColumns = ref<Array<Column>>();

onBeforeMount(() => {
    localColumns.value = parents.columns;
    row.value = parents.info;
});

function onSave() {
    emit("update:info", row.value);
    emit("onSave");
}
function onCancel() {
    emit("onCancel");
}

function selectIcon(param: any) {
    row.value[param.columnName] = param.icon;
}

// 查询筛选
function handleSearch() {
    // TODO 多个筛选框互相影响时处理
    emit("searchTable", localSearchParameters.value);
}

watch(
    () => parents.columns,
    (newValue, oldValue) => {
        localColumns.value = newValue;
    }
);
</script>
<style scoped>
#Table {}
</style>
