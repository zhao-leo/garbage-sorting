use serialport::{self, SerialPort};
use std::thread::sleep;
// use std::thread::sleep;
use std::time::Duration;
use std::env;
use std::io::{self, Read};

// 在文件开头添加宏定义
#[macro_export]
macro_rules! POSITION_A {
    () => {
        Coordinate { x: 100.0, y: 100.0 }
    };
}

#[macro_export]
macro_rules! POSITION_B {
    () => {
        Coordinate { x: 200.0, y: 100.0 }
    };
}

#[macro_export]
macro_rules! POSITION_C {
    () => {
        Coordinate { x: 100.0, y: 200.0 }
    };
}

#[macro_export]
macro_rules! POSITION_D {
    () => {
        Coordinate { x: 200.0, y: 200.0 }
    };
}

pub struct SerialPortHandler {
    port1: Box<dyn SerialPort>,
    port2: Box<dyn SerialPort>,
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

impl SerialPortHandler {
    /// 从环境变量获取串口配置
    pub fn from_env() -> Result<(Self, f32, f32), Box<dyn std::error::Error>> {
        let ports_var = env::var("SERIAL_PORTS").unwrap_or("/dev/ttyS0,/dev/ttyS1".to_string());
        let ports: Vec<&str> = ports_var.split(',').collect();
        //export SERIAL_PORTS="/dev/ttyACM0,/dev/ttyACM1"  第一个是port_location 第二个是port_flag
        //CAMERA_DISTANCE_Y CAMERA_DISTANCE_X，设置成mm单位
        // 获取相机 X 轴距离配置
        let camera_distance_x = env::var("CAMERA_DISTANCE_X")
            .unwrap_or("100.0".to_string())
            .parse::<f32>()
            .unwrap_or(100.0);  // 默认值为 100.0

        // 获取相机 Y 轴距离配置
        let camera_distance_y = env::var("CAMERA_DISTANCE_Y")
            .unwrap_or("100.0".to_string())
            .parse::<f32>()
            .unwrap_or(100.0);  // 默认值为 100.0

        if ports.len() != 2 {
            return Err("需要配置两个串口设备".into());
        }
        
        let handler = Self::new(ports[0], ports[1])?;
        Ok((handler, camera_distance_x, camera_distance_y))
    }

    /// 初始化两个串口
    fn new(port_location: &str, port_flag: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let port1 = serialport::new(port_location, 115200)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .timeout(Duration::from_secs(1))
            .open()?;

        let port2 = serialport::new(port_flag, 115200)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .timeout(Duration::from_secs(1))
            .open()?;

        Ok(Self { port1, port2 })
    }

    /// 发送初始化命令
    pub fn send_init_commands(&mut self) -> Result<(), std::io::Error> {
        let commands = "G0 G21 G17 G90";
        let message = format!("{}\r\n", commands);
        self.port1.write_all(message.as_bytes())?;
        sleep(Duration::from_millis(500));
        println!("已发送初始化命令: {}", commands);
        let response = self.read_response()?;
        println!("读取responce{}", response);
        Ok(())
    }

    /// 发送坐标到串口1
    pub fn send_location(&mut self, coord: &Coordinate) -> Result<(), std::io::Error> {
        // let solution = format!("$X\r\n");
        // self.port1.write_all(solution.as_bytes())?;
        // sleep(Duration::from_secs(1));
        let message = format!("G0 X{:.2} Y{:.2}\r\n", -coord.x, coord.y);
        sleep(Duration::from_secs(1));
        self.port1.write_all(message.as_bytes())?;
        let response = self.read_response()?;
        println!("读取responce{}", response);
        Ok(())
    }

    /// 发送标志到串口2
    pub fn send_flag(&mut self, task_type: i32, flag: i32) -> Result<(), std::io::Error> {
        let message = format!("{},{}\r\n", task_type, flag);
        self.port2.write_all(message.as_bytes())?;
        Ok(())
    }

    /// 发送归零命令 $H
    pub fn send_home_command(&mut self) -> Result<(), std::io::Error> {
        for attempt in 1..=2 {
            println!("发送归零命令 $H (尝试 {})", attempt);
            self.port1.write_all(b"$H\r\n")?;
            
            // 读取响应
            let response = self.read_response()?;
            if response.contains("error") {
                println!("归零命令执行失败，响应: {}", response);
                if attempt == 2 {
                    return Err(io::Error::new(io::ErrorKind::Other, "归零操作故障"));
                }
                continue;
            }

            if response.contains("ok") {
                println!("归零命令执行成功");
                return Ok(());
            }

            if attempt == 2 {
                return Err(io::Error::new(io::ErrorKind::Other, "未收到正确的响应"));
            }
        }
        
        Err(io::Error::new(io::ErrorKind::Other, "归零操作失败"))
    }

    /// 读取串口响应
    fn read_response(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = [0u8; 1024];
        let mut response = String::new();
        let start_time = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(10);
        
        loop {
            if start_time.elapsed() >= timeout_duration {
                return Err(io::Error::new(io::ErrorKind::TimedOut, "读取响应超时"));
            }

            match self.port1.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    response.push_str(&String::from_utf8_lossy(&buffer[..n]));
                    if response.trim().ends_with("ok") {
                        return Ok(response);
                    }
                }
                Ok(_) => continue,
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
                Err(e) => return Err(e),
            }
        }
    }

    /// 归位和置零函数
    pub fn home_and_reset(&mut self) -> Result<(), std::io::Error> {
        // 第一步：执行归零操作
        println!("开始执行归零操作...");
        for attempt in 1..=2 {
            println!("发送归零命令 $H (第{}次尝试)", attempt);
            self.port1.write_all(b"$H\r\n")?;
            
            match self.read_response() {
                Ok(response) => {
                    println!("归零命令响应: {}", response.trim());
                    if response.contains("ok") {
                        println!("归零命令执行成功");
                        break;
                    }
                    if attempt == 2 {
                        return Err(io::Error::new(io::ErrorKind::Other, "归零操作两次尝试均失败"));
                    }
                },
                Err(e) => {
                    if attempt == 2 {
                        return Err(io::Error::new(io::ErrorKind::Other, 
                            format!("读取归零响应失败: {}", e)));
                    }
                }
            }
            println!("第{}次归零尝试失败，准备重试...", attempt);
            sleep(Duration::from_secs(1));
        }

        // 第二步：执行置零操作
        println!("\n开始执行置零操作...");
        for attempt in 1..=2 {
            println!("发送置零命令 G92 X0 Y0 Z0 (第{}次尝试)", attempt);
            self.port1.write_all(b"G92 X0 Y0 Z0\r\n")?;
            
            match self.read_response() {
                Ok(response) => {
                    println!("置零命令响应: {}", response.trim());
                    if response.contains("ok") {
                        println!("置零命令执行成功");
                        println!("归位和置零操作全部完成");
                        return Ok(());
                    }
                    if attempt == 2 {
                        return Err(io::Error::new(io::ErrorKind::Other, "置零操作两次尝试均失败"));
                    }
                },
                Err(e) => {
                    if attempt == 2 {
                        return Err(io::Error::new(io::ErrorKind::Other, 
                            format!("读取置零响应失败: {}", e)));
                    }
                }
            }
            println!("第{}次置零尝试失败，准备重试...", attempt);
            sleep(Duration::from_secs(1));
        }
        
        Err(io::Error::new(io::ErrorKind::Other, "归位和置零操作完全失败"))
    }

    /// 向串口2发送p字符
    pub fn send_p(&mut self) -> Result<(), std::io::Error> {
        self.port2.write_all(b"p\r\n")?;
        println!("已发送p字符");
        Ok(())
    }

    /// 显式关闭串口
    pub fn close(self) {
        drop(self.port1);
        drop(self.port2);
    }
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let (mut handler, _, _) = SerialPortHandler::from_env()?;
    handler.home_and_reset()?;
    handler.send_init_commands()?;
    println!("串口初始化完成");
    Ok(())
}

// 任务一函数，只负责传递任务类型和垃圾种类
pub fn process_serial_communication_task1(
    flag: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let delay = Duration::from_secs(6);  // 任务1等待舵机操作，后期修改
    // 初始化串口
    let (mut handler,_,_) = SerialPortHandler::from_env()?;
    // 发送任务类型1和标志
    handler.send_flag(1, flag)?;
    println!("已发送任务1, 标志: {}", flag);
    // 等待任务完成
    std::thread::sleep(delay);
    println!("任务1完成");
    // 关闭串口
    handler.close();
    
    Ok(())
}


pub fn process_serial_communication_task2(
    coordinate: Coordinate,
    flag: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let delay_motor=Duration::from_secs(8);
    // 根据 position_type 选择坐标
    let throw_coordinate = match flag {
        1 => POSITION_A!(),
        2 => POSITION_B!(),
        3 => POSITION_C!(),
        4 => POSITION_D!(),
        _ => return Err("无效的位置类型".into()),
    };
    // 初始化串口
    let (mut handler, camera_distance_x, camera_distance_y) = SerialPortHandler::from_env()?;
    let real_coordinate = Coordinate {
        x: coordinate.x * (camera_distance_x / 640.0),
        y: coordinate.y * (camera_distance_y / 640.0),
    };

    // 发送坐标给步进电机
    handler.send_location(&real_coordinate)?;
    println!("已发送坐标: {:?}", coordinate);

    // 延时等待
    std::thread::sleep(delay_motor);

    // 发送标志
    handler.send_flag(2, flag)?;
    println!("已发送给mega标志2和flag: {}", flag);
    // 等待步进电机运动完成
    std::thread::sleep(Duration::from_secs(3));//等待舵机

    // 发送扔垃圾坐标
    handler.send_location(&throw_coordinate)?;
    print!("已发送第{}个扔垃圾坐标",flag);
    //等待扔完垃圾
    std::thread::sleep(Duration::from_secs(8));
    //发送停止标志
    handler.send_p()?;
    std::thread::sleep(Duration::from_secs(2));
    handler.send_home_command()?;
    println!("归零操作完成");
    // 关闭串口
    handler.close();
    
    Ok(())
}