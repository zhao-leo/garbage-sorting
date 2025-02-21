use tauri::{command, AppHandle};

mod camera;
mod serial;
#[command]
async fn listen_serial_port(app: AppHandle) -> Result<(), String> {
    serial::listen_serial_port(app).await
}

#[command]
fn capture_and_save(file_path: &str) -> Result<bool, String> {
    camera::capture_and_save(file_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            listen_serial_port,
            capture_and_save
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
