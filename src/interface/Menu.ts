export interface MenuItem {
    title?: String,
    index: String,
    text: String,
    link?: String,
    icon?: String,
    data?: Array<MenuItem>,
    width?: string,
}

export interface Menu {
    index?: String,
    text: String,
    activeIndex?: String, // 默认页面 index
    class?: String,
    mode?: String,
    icon?: String,
    group?: Array<MenuItem>,
    data: Array<MenuItem>,
    width?: string,

}
export interface Aside {
    activeIndex?: String | Array<String>,
    submenu?: Array<Menu | MenuItem>,
    openedsIndex?: Array<String>,

}

export interface Breadcrumb {
    path?: String,
    text: String
}
