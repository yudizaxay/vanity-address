mod commands;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::list_chains,
            commands::get_system_profile,
            commands::estimate,
            commands::start_grind,
            commands::stop_grind,
            commands::save_result,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
