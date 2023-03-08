<template>
    <el-button text @click="dialogVisible = true" :icon="parents.icon" :type="parents.type" size="small"> 点击修改图标</el-button>
    <el-dialog v-model="dialogVisible" title="图标列表选择" width="60%" :append-to-body="true">
        <div>
            <ul class="item-list">
                <li v-for="(ita, index) in iconList" :key="index" @click="selectIcon(ita)" :class="ita == parents.icon ? 'select' : 'item'">
                    <el-icon :size="18">
                        <component :is="ita"></component>
                    </el-icon>
                    <span>{{ ita }} </span>
                </li>
            </ul>
        </div>
    </el-dialog>
</template>

<script lang="ts" setup>
// import { defineComponent } from "vue";
import { defineEmits, ref, onBeforeMount } from "vue";

const parents = withDefaults(
    defineProps<{
        icon: string;
        // columnName?: string;
        type?: string;
    }>(),
    {
        icon: () => "",
        type: () => "",
    }
);

let loicon = ref<string>();

const emit = defineEmits(["update:icon"]);
const dialogVisible = ref(false);

const iconList = ["AddLocation", "Aim", "AlarmClock", "Apple", "ArrowDown", "ArrowDownBold", "ArrowLeft", "ArrowLeftBold", "ArrowRight", "ArrowRightBold", "ArrowUp", "ArrowUpBold", "Avatar", "Back", "Baseball", "Basketball", "Bell", "BellFilled", "Bicycle", "Bottom", "BottomLeft", "BottomRight", "Bowl", "Box", "Briefcase", "Brush", "BrushFilled", "Burger", "Calendar", "Camera", "CameraFilled", "CaretBottom", "CaretLeft", "CaretRight", "CaretTop", "Cellphone", "ChatDotRound", "ChatDotSquare", "ChatLineRound", "ChatLineSquare", "ChatRound", "ChatSquare", "Check", "Checked", "Cherry", "Chicken", "CircleCheck", "CircleCheckFilled", "CircleClose", "CircleCloseFilled", "CirclePlus", "CirclePlusFilled", "Clock", "Close", "CloseBold", "Cloudy", "Coffee", "CoffeeCup", "Coin", "ColdDrink", "Collection", "CollectionTag", "Comment", "Compass", "Connection", "Coordinate", "CopyDocument", "Cpu", "CreditCard", "Crop", "DArrowLeft", "DArrowRight", "DCaret", "DataAnalysis", "DataBoard", "DataLine", "Delete", "DeleteFilled", "DeleteLocation", "Dessert", "Discount", "Dish", "DishDot", "Document", "DocumentAdd", "DocumentChecked", "DocumentCopy", "DocumentDelete", "DocumentRemove", "Download", "Drizzling", "Edit", "EditPen", "Eleme", "ElemeFilled", "ElementPlus", "Expand", "Failed", "Female", "Files", "Film", "Filter", "Finished", "FirstAidKit", "Flag", "Fold", "Folder", "FolderAdd", "FolderChecked", "FolderDelete", "FolderOpened", "FolderRemove", "Food", "Football", "ForkSpoon", "Fries", "FullScreen", "Goblet", "GobletFull", "GobletSquare", "GobletSquareFull", "Goods", "GoodsFilled", "Grape", "Grid", "Guide", "Headset", "Help", "HelpFilled", "Hide", "Histogram", "HomeFilled", "HotWater", "House", "IceCream", "IceCreamRound", "IceCreamSquare", "IceDrink", "IceTea", "InfoFilled", "Iphone", "Key", "KnifeFork", "Lightning", "Link", "List", "Loading", "Location", "LocationFilled", "LocationInformation", "Lock", "Lollipop", "MagicStick", "Magnet", "Male", "Management", "MapLocation", "Medal", "Menu", "Message", "MessageBox", "Mic", "Microphone", "MilkTea", "Minus", "Money", "Monitor", "Moon", "MoonNight", "More", "MoreFilled", "MostlyCloudy", "Mouse", "Mug", "Mute", "MuteNotification", "NoSmoking", "Notebook", "Notification", "Odometer", "OfficeBuilding", "Open", "Operation", "Opportunity", "Orange", "Paperclip", "PartlyCloudy", "Pear", "Phone", "PhoneFilled", "Picture", "PictureFilled", "PictureRounded", "PieChart", "Place", "Platform", "Plus", "Pointer", "Position", "Postcard", "Pouring", "Present", "PriceTag", "Printer", "Promotion", "QuestionFilled", "Rank", "Reading", "ReadingLamp", "Refresh", "RefreshLeft", "RefreshRight", "Refrigerator", "Remove", "RemoveFilled", "Right", "ScaleToOriginal", "School", "Scissor", "Search", "Select", "Sell", "SemiSelect", "Service", "SetUp", "Setting", "Share", "Ship", "Shop", "ShoppingBag", "ShoppingCart", "ShoppingCartFull", "Smoking", "Soccer", "SoldOut", "Sort", "SortDown", "SortUp", "Stamp", "Star", "StarFilled", "Stopwatch", "SuccessFilled", "Sugar", "Suitcase", "Sunny", "Sunrise", "Sunset", "Switch", "SwitchButton", "TakeawayBox", "Ticket", "Tickets", "Timer", "ToiletPaper", "Tools", "Top", "TopLeft", "TopRight", "TrendCharts", "Trophy", "TurnOff", "Umbrella", "Unlock", "Upload", "UploadFilled", "User", "UserFilled", "Van", "VideoCamera", "VideoCameraFilled", "VideoPause", "VideoPlay", "View", "Wallet", "WalletFilled", "Warning", "WarningFilled", "Watch", "Watermelon", "WindPower", "ZoomIn", "ZoomOut"];

onBeforeMount(() => {
    // loicon.value = parents.icon;
});

function selectIcon(ita: string) {
    emit("update:icon", ita);
    // parents.icon = ita;
}
</script>

<style lang="scss" scoped>
.item-list {
    overflow: hidden;
    list-style: none;
    padding: 0 !important;
    border-top: 1px solid var(--el-border-color);
    border-left: 1px solid var(--el-border-color);
    border-radius: 4px;
    display: grid;
    grid-template-columns: repeat(4, 1fr);

    .item {
        padding-top: 5px;
        // border-radius: 5px;
        text-align: left;
        color: var(--el-text-color-regular);
        height: 30px;
        font-size: 13px;
        border-right: 1px solid var(--el-border-color);
        border-bottom: 1px solid var(--el-border-color);
        transition: background-color var(--el-transition-duration);
    }

    .select {
        padding-top: 5px;
        // border-radius: 5px;
        text-align: left;
        color: var(--el-text-color-regular);
        height: 30px;
        font-size: 13px;
        border-right: 1px solid var(--el-border-color);
        border-bottom: 1px solid var(--el-border-color);
        transition: background-color var(--el-transition-duration);
        background-color: #409eff;
    }

    .item:hover {
        background-color: rgb(238, 255, 255);
        cursor: pointer;
    }
}
</style>
