#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::info;
use rusqlite::Connection;
use std::env;
use tauri::{Manager, State};
use tauri_plugin_log::LogTarget;
use teller::handlers::config::get_config_folder;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
}

fn main() {

    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = teller_desktop::backend::database_handler::initialize_database(&handle)
                .expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            // Run migrations
            if let Ok(mut db) = app_state.db.lock() {
                if let Some(ref mut db_conn) = *db {
                    let migrations = include_str!("../../../migrations/20231128193314_init.sql");
                    let current_version: u32 = db_conn.pragma_query_value(None, "user_version", |row| row.get(0)).unwrap_or(0);
                    if current_version < teller_desktop::backend::database_handler::CURRENT_DB_VERSION {
                        let _ = db_conn.execute_batch(migrations); // Ignore if migrations have already been applied
                        let _ = teller_desktop::backend::database_handler::upgrade_database_if_needed(db_conn, current_version); // Ignore if upgrade is not needed
                    }
                }
            }

            Ok(())
        })
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
