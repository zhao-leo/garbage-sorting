use tokio::io::AsyncReadExt;
use tokio_serial::{SerialPortBuilder, SerialStream,SerialPortBuilderExt};
use tokio_serial::Error as SerialError;
/// 异步监听串口数据的函数
///
/// # 参数
/// - `port_builder`: 已配置的串口构建器（包含路径、波特率等参数）
/// - `callback`: 数据处理回调函数，接收字节切片作为输入
///
/// # 返回
/// - `Result<(), SerialError>`: 成功时返回 `Ok(())`，出错时返回串口错误
pub async fn listen_serial(
    port_builder: SerialPortBuilder,
    mut callback: impl FnMut(&[u8]),
) -> Result<(), SerialError> {
    // 打开异步串口流
    let mut port: SerialStream = port_builder.open_native_async()?;

    // 设置读取缓冲区大小（根据需求调整）
    let mut buf = [0; 1024];

    loop {
        // 异步读取数据
        match port.read(&mut buf).await {
            Ok(n) => {
                // 调用回调处理接收到的数据
                callback(&buf[..n]);
            }
            Err(e) => {
                // 发生错误时返回（如连接断开）
                return Err(e.into());
            }
        }
    }
}

// #[tokio::main]
// async fn main() -> Result<(), SerialError> {
//     // 配置串口参数
//     let builder = tokio_serial::new("COM4", 115200)
//         .data_bits(tokio_serial::DataBits::Eight)
//         .stop_bits(tokio_serial::StopBits::One)
//         .parity(tokio_serial::Parity::None);

//     // 调用监听函数，处理接收到的数据
//     listen_serial(builder, |data| {
//         println!("Received: {:?}", data.to_vec().iter().map(|&c| c as char).collect::<String>());
//     }).await?;

//     Ok(())
// }