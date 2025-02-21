use opencv::{
    core::Mat,
    imgcodecs,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};
use std::{env, result::Result, string::String};
pub fn capture_and_save() -> Result<bool, String> {
    let file_path = env::var("TEMP_DIR").unwrap() + "/capture.png";
    // 初始化摄像头，0表示默认摄像头
    let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();

    if !cam.is_opened().unwrap() {
        return Ok(false);
    }

    let mut frame = Mat::default();

    cam.read(&mut frame).unwrap();

    if frame.empty() {
        return Ok(false);
    }

    imgcodecs::imwrite(&file_path, &frame, &opencv::core::Vector::new()).unwrap();

    Ok(true)
}
