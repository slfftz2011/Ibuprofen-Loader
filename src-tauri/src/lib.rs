// Main entry point for Tauri application
mod models;
mod commands;
mod services;
mod utils;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();
    info!("Starting Ibuprofen Loader v3.0.0");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Registry commands
            commands::registry::get_netease_download_path,
            // Network commands
            commands::network::check_connection_status,
            commands::network::check_website_reachable,
            // File processor commands
            commands::file::scan_cop_files,
            commands::file::process_cop_file,
            commands::file::get_components_directory,
            commands::file::check_or_create_components_dir,
            // JSON parser commands
            commands::json::load_list_json,
            commands::json::verify_file_exists,
            // Injector commands
            commands::injection::start_injection,
            commands::injection::backup_directories,
            commands::injection::wait_for_game_launch,
            commands::injection::copy_files_to_game,
            // URL commands
            commands::url::open_url,
            // System commands
            commands::system::get_app_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
