use tauri::{command, AppHandle, Emitter};
use tokio_serial;
mod serial;
use serial::listen_serial;

#[command]
async fn listen_serial_port(app:AppHandle) -> Result<(), String> {
    println!("Listening to serial port");
    let builder = tokio_serial::new("COM7", 9600)
        .data_bits(tokio_serial::DataBits::Eight)
        .stop_bits(tokio_serial::StopBits::One)
        .parity(tokio_serial::Parity::None);

        listen_serial(builder, move |data| {
            let received_data = data.to_vec().iter().map(|&c| c as char).collect::<String>();
            println!("Received: {:?}", received_data);
            if received_data.trim() == "ok" {
                println!("Received OK signal, stopping...");
                app.emit("serial-data-received", received_data).unwrap();
                return true; // 返回 表示停止监听
            }
            false // 返回 表示继续监听
        }).await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![listen_serial_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
