import { PageParam, SelectOptions } from "@interface/Table";
import { SysApplTypeTabsColumnInfoVo } from "./SysApp";
import { SysDbmsTabsCols } from "./SysDbms";

export interface SearchIndexParameters extends SelectOptions {
  indexPlaceholder?: string;
  data?: string | number | boolean | undefined;
}


export interface TabsPageParams<T> extends PageParam<T> {
  cols?: Array<SysDbmsTabsCols | SysApplTypeTabsColumnInfoVo>;
}