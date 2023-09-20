#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::info;
use std::env;
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use teller::handlers::config::get_config_folder;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    LogTarget::Folder(get_config_folder()),
                    LogTarget::Stdout,
                    LogTarget::Webview,
                ])
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            info!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(teller_desktop::config::init())
        .plugin(teller_desktop::backend::backup_handler::init())
        .plugin(teller_desktop::backend::folder_handler::init())
        .plugin(teller_desktop::backend::world_handler::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
