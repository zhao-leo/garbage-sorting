// use tauri::{command, AppHandle};
use tauri::command;
use yolo_binding::core::YOLO;
extern crate yolo_binding;
mod camera;
mod serial;
mod predict;
mod xy_core;

use lazy_static::lazy_static;

use tokio::sync::Mutex;

lazy_static! {
    static ref MODEL: Mutex<Option<YOLO>> = Mutex::new(None);
}

// #[command]
// async fn listen_serial_port(app: AppHandle) -> Result<(), String> {
//     serial::listen_serial_port(app).await
// }

#[command]
fn capture_and_save() -> Result<bool, String> {
    camera::capture_and_save()
}

#[command]
async fn initialize_model() -> Result<(), String> {
    let model = predict::load_model();
    *MODEL.lock().await = Some(model);
    Ok(())
}

#[command]
fn xy_init() -> Result<(), String>{
    xy_core::init().map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
fn broad(flag:i32) -> Result<(), String>{
    xy_core::process_serial_communication_task1(flag).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
fn xy_run(x:f32,y:f32,flag:i32) -> Result<(), String>{
    xy_core::process_serial_communication_task2(xy_core::Coordinate{x,y},flag).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
async fn predict_image() -> Result<(i64, i64, i64,i64), String> {
    let model = MODEL.lock().await;
    let model = model.as_ref().ok_or("Model not initialized")?;
    predict::predict_image(model).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // listen_serial_port,
            capture_and_save,
            predict_image,
            initialize_model,
            xy_init,
            xy_run,
            broad,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
