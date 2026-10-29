pub mod models;
pub mod cloud;
pub mod db;
pub mod drive;
pub mod metadata;
pub mod runner;
pub mod scanners;
pub mod tray;

use models::Game;
use std::collections::HashMap;

// ============================================================================
// COMMANDES DU DEV 2 (Système local, SQLite & Launchers PC — 9 commandes)
// ============================================================================

#[tauri::command]
fn get_all_games() -> Result<Vec<Game>, String> {
    Ok(vec![]) // Dev 2 branchera db::get_all_games() ici (DEV2-01)
}

#[tauri::command]
async fn scan_all_sources() -> Result<Vec<Game>, String> {
    Ok(vec![]) // Dev 2 branchera scanners::local::scan_all_sources() + Dev 3 y ajoutera Drive/Steam Cloud (DEV2-04)
}

#[tauri::command]
async fn launch_game(game_id: String) -> Result<(), String> {
    let _ = game_id;
    Ok(()) // Dev 2 branchera runner::launch_game() ici (DEV2-09)
}

#[tauri::command]
async fn uninstall_game(game_id: String) -> Result<(), String> {
    let _ = game_id;
    Ok(()) // Dev 2 branchera runner::uninstall::uninstall_game() ici (DEV2-11)
}

#[tauri::command]
fn set_game_executable(game_id: String, exe_path: String) -> Result<(), String> {
    let _ = (game_id, exe_path);
    Ok(()) // Dev 2 branchera db::set_game_executable() ici (DEV2-01)
}

    #[tauri::command]
    fn add_manual_game(title: String, exe_path: String) -> Result<Game, String> {
        let _ = (title, exe_path);
        Err("Non implémenté (DEV2-10)".to_string()) // Dev 2 branchera runner::utils::add_manual_game() ici
    }

#[tauri::command]
fn open_game_folder(game_id: String) -> Result<(), String> {
    let _ = game_id;
    Ok(()) // Dev 2 branchera runner::utils::open_game_folder() ici (DEV2-10)
}

#[tauri::command]
fn get_settings() -> Result<HashMap<String, String>, String> {
    Ok(HashMap::new()) // Dev 2 branchera db::get_settings() ici (DEV2-01)
}

#[tauri::command]
fn set_setting(key: String, value: String) -> Result<(), String> {
    let _ = (key, value);
    Ok(()) // Dev 2 branchera db::set_setting() ici (DEV2-01)
}

// ============================================================================
// COMMANDES DU DEV 3 (Réseau, Google Drive & Cloud — 4 commandes)
// ============================================================================

#[tauri::command]
async fn get_network_status() -> Result<bool, String> {
    cloud::network::get_network_status().await
}

#[tauri::command]
async fn start_drive_download(game_id: String) -> Result<(), String> {
    let _ = game_id;
    Ok(()) // Dev 3 branchera drive::downloader::start_drive_download() ici (DEV3-05)
}

#[tauri::command]
async fn cancel_drive_download(game_id: String) -> Result<(), String> {
    let _ = game_id;
    Ok(()) // Dev 3 branchera drive::downloader::cancel_drive_download() ici (DEV3-05)
}

#[tauri::command]
async fn connect_google_drive() -> Result<(), String> {
    Ok(()) // Dev 3 branchera drive::auth::connect_google_drive() ici (DEV3-03)
}

// ============================================================================
// POINT D'ENTRÉE TAURI (Enregistrement des 13 commandes)
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // .clone() sur AppHandle est justifié car il doit être déplacé ("moved") dans la tâche asynchrone
            // tout en laissant `app` disponible pour la suite du setup. (Cloner un AppHandle est très léger : c'est un pointeur interne).
            cloud::network::start_network_monitor(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Dev 2
            get_all_games,
            scan_all_sources,
            launch_game,
            uninstall_game,
            set_game_executable,
            add_manual_game,
            open_game_folder,
            get_settings,
            set_setting,
            // Dev 3
            get_network_status,
            start_drive_download,
            cancel_drive_download,
            connect_google_drive,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}