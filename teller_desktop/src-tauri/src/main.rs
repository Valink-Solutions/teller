#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use tauri_plugin_log::LogTarget;
use teller::configuration::get_config_folder;

fn main() {
    let config_dir = get_config_folder();
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    LogTarget::Folder(config_dir),
                    LogTarget::Stdout,
                    LogTarget::Webview,
                ])
                .build(),
        )
        .plugin(teller_desktop::config::init())
        .plugin(teller_desktop::backend::folder_handler::init())
        .plugin(teller_desktop::backend::world_handler::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
