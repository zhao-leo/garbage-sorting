use opencv::{
    core::{self, Mat, Point2f, Size},
    imgcodecs, imgproc,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
    Result,
};
use std::{env, string::String};
macro_rules! CORNER_TOP_LEFT {
    () => {
        Point2f::new(200.0, 95.0)
    };
}

macro_rules! CORNER_TOP_RIGHT {
    () => {
        Point2f::new(440.0, 95.0)
    };
}

macro_rules! CORNER_BOTTOM_LEFT {
    () => {
        Point2f::new(200.0, 320.0)
    };
}

macro_rules! CORNER_BOTTOM_RIGHT {
    () => {
        Point2f::new(440.0, 320.0)
    };
}

// 目标矫正后的尺寸
macro_rules! TARGET_WIDTH {
    () => {
        640.0
    };
}

macro_rules! TARGET_HEIGHT {
    () => {
        640.0
    };
}

// 添加透视校正函数
fn perspective_correction(input: &Mat) -> Result<Mat> {
    // 定义源点（图像中实际的四个角点）
    let src_points = core::Vector::<Point2f>::from_slice(&[
        CORNER_TOP_LEFT!(),
        CORNER_TOP_RIGHT!(),
        CORNER_BOTTOM_LEFT!(),
        CORNER_BOTTOM_RIGHT!(),
    ]);

    // 定义目标点（期望校正后的四个角点）
    let dst_points = core::Vector::<Point2f>::from_slice(&[
        Point2f::new(0.0, 0.0),
        Point2f::new(TARGET_WIDTH!(), 0.0),
        Point2f::new(0.0, TARGET_HEIGHT!()),
        Point2f::new(TARGET_WIDTH!(), TARGET_HEIGHT!()),
    ]);

    // 计算透视变换矩阵
    let transform_matrix =
        imgproc::get_perspective_transform(&src_points, &dst_points, core::DECOMP_LU)?;

    // 应用透视变换
    let mut output = Mat::default();
    imgproc::warp_perspective(
        input,
        &mut output,
        &transform_matrix,
        Size::new(TARGET_WIDTH!() as i32, TARGET_HEIGHT!() as i32),
        imgproc::INTER_LINEAR,
        core::BORDER_CONSTANT,
        core::Scalar::default(),
    )?;

    Ok(output)
}

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
    let corrected_frame = match perspective_correction(&frame) {
        Ok(corrected) => corrected,
        Err(_) => frame, // 如果校正失败，使用原始帧
    };

    imgcodecs::imwrite(&file_path, &corrected_frame, &opencv::core::Vector::new()).unwrap();

    Ok(true)
}

pub fn get_basic_image() -> Result<bool, String> {
    let file_path = env::var("TEMP_DIR").unwrap() + "/basic.png";
    let raw_path = env::var("TEMP_DIR").unwrap() + "/raw.png";
    let mut cam = VideoCapture::new(0, CAP_ANY).unwrap();

    if !cam.is_opened().unwrap() {
        return Ok(false);
    }

    let mut frame = Mat::default();

    cam.read(&mut frame).unwrap();

    if frame.empty() {
        return Ok(false);
    }
    cam.read(&mut frame).unwrap();

    imgcodecs::imwrite(&raw_path, &frame, &opencv::core::Vector::new()).unwrap();

    let corrected_frame = match perspective_correction(&frame) {
        Ok(corrected) => corrected,
        Err(_) => frame, // 如果校正失败，使用原始帧
    };

    imgcodecs::imwrite(&file_path, &corrected_frame, &opencv::core::Vector::new()).unwrap();

    Ok(true)
}

pub fn similarity(threshold: f64) -> Result<bool> {
    let temp_dir = env::var("TEMP_DIR").unwrap();
    let basic_path = temp_dir.clone() + "/basic.png";
    let capture_path = temp_dir.clone() + "/capture.png";
    let basic = imgcodecs::imread(&basic_path, imgcodecs::IMREAD_COLOR)?;
    let capture = imgcodecs::imread(&capture_path, imgcodecs::IMREAD_COLOR)?;

    // 统一图片尺寸（以 basic 为基准调整 capture）
    let basic_size = basic.size()?;
    let mut resized_capture = core::Mat::default();
    imgproc::resize(
        &capture,
        &mut resized_capture,
        basic_size,
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )?;

    // 转换为灰度图
    let mut basic_gray = core::Mat::default();
    let mut capture_gray = core::Mat::default();
    imgproc::cvt_color(
        &basic,
        &mut basic_gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        1,
    )?;
    imgproc::cvt_color(
        &resized_capture,
        &mut capture_gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        1,
    )?;

    // 计算像素差异
    let mut diff = core::Mat::default();
    core::absdiff(&basic_gray, &capture_gray, &mut diff)?;

    // 转换为浮点类型防止计算溢出
    let mut diff_float = core::Mat::default();
    diff.convert_to(&mut diff_float, core::CV_32F, 1.0, 0.0)?;

    // 计算平方差
    let mut diff_sq = core::Mat::default();
    core::multiply(&diff_float, &diff_float, &mut diff_sq, 1.0, -1)?;

    // 计算均方误差（MSE）
    let mask = core::no_array();
    let mse = core::mean(&diff_sq, &mask)?[0] as f64;

    // 计算相似度百分比（基于最大可能 MSE）
    const MAX_PIXEL_VALUE: f64 = 255.0;
    let max_mse = MAX_PIXEL_VALUE.powi(2);
    let similarity = (1.0 - mse / max_mse) * 100.0;
    // 加大相似度差异
    let similarity = (similarity - 80.0)/20.0;
    println!("max_mase:{}\nsimilarity:{}", max_mse, similarity);
    // 返回比较结果
    Ok(similarity < threshold)
}
