// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn set_window_decorations(decorations: bool, window: tauri::Window) -> tauri::Result<()> {
    window.set_decorations(decorations)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_window_decorations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
