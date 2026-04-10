pub mod commands;
pub mod events;
pub mod models;
pub mod runtime;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(runtime::state::AppRuntimeState::new())
        .invoke_handler(tauri::generate_handler![
            commands::runtime::get_runtime_snapshot,
            commands::runtime::initialize_runtime_state,
            commands::runtime::ping_runtime
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
