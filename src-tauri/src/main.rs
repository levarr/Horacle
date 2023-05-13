// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    Window,
};
use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(SystemTray::new().with_menu(build_menu()))
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick { .. } => {
                    let window = app.get_window("main").unwrap();
                    toggle_app_visibility(&window)
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "show_hide" => {
                        let window = app.get_window("main").unwrap();
                        toggle_app_visibility(&window);
                    }
                    "quit" => std::process::exit(0),
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(false) => event.window().hide().unwrap(),
            _ => {}
        })
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let window = app.get_window("main").unwrap();
            window.set_always_on_top(true).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_menu() -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(
            "show_hide".to_string(),
            "Show/Hide Horacle",
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q"))
}

fn toggle_app_visibility(window: &Window) -> () {
    if !window.is_visible().unwrap() {
        let _ = window.move_window(Position::TrayCenter).map(|()| {
            window.show().unwrap();
            window.set_focus().unwrap();
        });
    } else { window.hide().unwrap(); }
}
