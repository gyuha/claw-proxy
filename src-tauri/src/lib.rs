pub mod commands;
pub mod events;
pub mod models;
pub mod runtime;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| -> Result<(), Box<dyn std::error::Error>> {
            let config_dir = app.path().app_config_dir()?;
            let state = runtime::state::AppRuntimeState::new_in_directory(&config_dir)?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::runtime::get_runtime_snapshot,
            commands::runtime::initialize_runtime_state,
            commands::runtime::ping_runtime,
            commands::proxy::get_proxy_runtime_snapshot,
            commands::proxy::get_proxy_settings,
            commands::proxy::start_proxy_runtime,
            commands::proxy::stop_proxy_runtime,
            commands::proxy::apply_proxy_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
