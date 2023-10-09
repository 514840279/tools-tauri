// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod base_dbutils;
use crate::base_dbutils::User;
use tracing::info;

// 菜单组件
use tauri::{
    AboutMetadata, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, SystemTray,
    SystemTrayMenu, SystemTrayMenuItem,
};

// 普通函数
#[tauri::command]
fn greet(name: &str) -> String {
    info!("从链接池获取数据库链接----------结束----------");
    let user:User = base_dbutils::select("  select uuid as id,name,sort as age from sys_menu_info where uuid='02E002A9D3F9BB1EFCE1B0D4AA333116'");
    println!("User id:{},name:{},age:{},", user.id, user.name, user.age);
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

// 菜单配置
fn main() {
    // 创建系统托盘菜单 退出和隐藏 todo 实现方法
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");

    // 创建系统托盘菜单
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    // 创建系统托盘
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        // 系统托盘
        .system_tray(system_tray)
        //新增菜单
        .menu(Menu::with_items([
            MenuEntry::Submenu(Submenu::new(
                "首页",
                Menu::with_items([
                    MenuItem::CloseWindow.into(),
                    #[cfg(target_os = "macos")]
                    CustomMenuItem::new("hello", "Hello").into(),
                ]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "我的任务",
                Menu::with_items([
                    MenuItem::CloseWindow.into(),
                    #[cfg(target_os = "macos")]
                    CustomMenuItem::new("hello", "Hello").into(),
                ]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "下载",
                Menu::with_items([
                    MenuItem::CloseWindow.into(),
                    #[cfg(target_os = "macos")]
                    CustomMenuItem::new("hello", "Hello").into(),
                ]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "关于我",
                Menu::with_items([
                    MenuItem::About(String::from("asd"), AboutMetadata::new()).into()
                ]),
            )),
        ]))
        // 启动画面
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();

            // 创建一个线程模拟页面准备中
            tauri::async_runtime::spawn(async move {
                // 初始化一个线程等待2s模拟页面准备中
                println!("模拟页面加载中");
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("页面加载结束");

                // 关闭启动画面，启动主窗口
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
                //   greet("5556")
            });
            Ok(())
        })
        // 窗口事件
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    //阻止默认关闭
                    // api.prevent_close();

                    // let _window = event.window().clone();
                    // tauri::confirm(Some(&event.window()), "关闭应用", "确定关闭当前应用?", move| answer|{
                    //     if answer {
                    //         let _result =window.close();
                    //     }
                    // })
                }
                _ => {} //todo
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
