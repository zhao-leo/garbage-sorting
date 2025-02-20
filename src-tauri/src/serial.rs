use tokio::io::AsyncReadExt;
// use tokio_serial::Error as SerialError;
use tokio_serial::{SerialPortBuilder, SerialPortBuilderExt, SerialStream};
/// 异步监听串口数据的函数
///
/// # 参数
/// - `port_builder`: 已配置的串口构建器（包含路径、波特率等参数）
/// - `callback`: 数据处理回调函数，接收字节切片作为输入
///
/// # 返回
/// - `Result<(), SerialError>`: 成功时返回 `Ok(())`，出错时返回串口错误
pub async fn listen_serial<F>(
    port_builder: SerialPortBuilder,
    mut callback: F,
) -> Result<(), String>
where
    F: FnMut(Vec<u8>) -> bool,
{
    // 打开异步串口流
    let mut port: SerialStream = port_builder.open_native_async().unwrap();

    // 设置读取缓冲区大小（根据需求调整）
    let mut buffer = [0; 1024];

    loop {
        let len = port.read(&mut buffer).await.map_err(|e| e.to_string())?;
        if len > 0 {
            if callback(buffer[..len].to_vec()) {
                return Ok(());
            }
        }
    }
}
