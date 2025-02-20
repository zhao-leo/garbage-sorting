use tauri::{command, AppHandle, Emitter};
use tokio_serial;
mod serial;
use serial::listen_serial;

#[command]
async fn listen_serial_port(window:AppHandle) -> Result<(), String> {
    // 配置串口参数
    println!("Listening to serial port");
    let builder = tokio_serial::new("COM7", 9600)
        .data_bits(tokio_serial::DataBits::Eight)
        .stop_bits(tokio_serial::StopBits::One)
        .parity(tokio_serial::Parity::None);

        listen_serial(builder, move |data| {
            let received_data = data.to_vec().iter().map(|&c| c as char).collect::<String>();
            println!("Received: {:?}", received_data);
    
            // 发送事件到前端
            window.emit("serial-data-received", received_data).unwrap();
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
