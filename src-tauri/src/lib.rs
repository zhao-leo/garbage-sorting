use tauri::{command, AppHandle};
extern crate yolo_binding;
mod camera;
mod serial;
mod predict;
#[command]
async fn listen_serial_port(app: AppHandle) -> Result<(), String> {
    serial::listen_serial_port(app).await
}

#[command]
fn capture_and_save() -> Result<bool, String> {
    camera::capture_and_save()
}

#[command]
async fn predict_image() -> Result<(i64, i64, i64), String> {
    predict::predict_image().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            listen_serial_port,
            capture_and_save,
            predict_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
