extern crate yolo_binding;

use tauri::{command, AppHandle};

use yolo_binding::core::YOLO;

mod camera;
mod serial;
mod predict;
mod xy_core;
mod rasp;

use lazy_static::lazy_static;

use tokio::sync::Mutex;

lazy_static! {
    static ref MODEL: Mutex<Option<YOLO>> = Mutex::new(None);
}
lazy_static! {
    static ref GPIO_MANAGER: Mutex<rasp::GpioManager> = Mutex::new(rasp::GpioManager::new());
}

// This function is called when the app is started
// #[command]
// async fn listen_serial_port(app: AppHandle) -> Result<(), String> {
//     /// 
//     serial::listen_serial_port(app).await
// }
#[command]
async fn initialize_model() -> Result<(), String> {
    /// Load the model and store it in the static variable
    let model = predict::load_model();
    *MODEL.lock().await = Some(model);
    Ok(())
}
#[command]
fn get_basic_image() -> Result<bool, String> {
    /// get reference image from camera
    camera::get_basic_image()
}
#[command]
fn xy_init() -> Result<(), String>{
    /// initialize the xy_core
    xy_core::init().map_err(|e| e.to_string())?;
    Ok(())
}

// This function is called on page-1
#[command]
fn similiarity() -> Result<bool,String> {
    /// the function would be called 5 seconds until detect the object
    camera::similarity(0.8).map_err(|x| {x.to_string()} )
}

// General function for predicting an image
#[command]
fn capture_and_save() -> Result<bool, String> {
    /// Capture an image and save it to the disk
    camera::capture_and_save()
}
#[command(async)]
async fn predict_image() -> Result<(i64, i64, i64,i64), String> {
    let model = MODEL.lock().await;
    let model = model.as_ref().ok_or("Model not initialized")?;
    predict::predict_image(model).await
}

// general function for serial port communication
#[command(async)]
pub async fn setup_gpio_monitoring(app_handle: tauri::AppHandle, pin: u8) -> Result<String, String> {

    let mut manager = GPIO_MANAGER.lock().await;
    manager.initialize(pin).map_err(|e| e.to_string())?;
    manager.start_monitoring(app_handle).map_err(|e| e.to_string())?;
    Ok(format!("Started monitoring GPIO pin {}", pin))
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



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // listen_serial_port,
            capture_and_save,
            get_basic_image,
            similiarity,
            predict_image,
            initialize_model,
            xy_init,
            xy_run,
            broad,
            listen_serial_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


    // let mut manager = rasp::GpioManager::new();
    // manager.initialize(pin)?;
    // manager.start_monitoring(app_handle)?;
  // // Add this function to your main.rs to register the command
// pub fn register_gpio_commands(app: &mut tauri::App) {
//     // Store the GpioManager in app state if needed for longer lifecycle
//     app.manage(Arc::new(Mutex::new(GpioManager::new())));
// }  