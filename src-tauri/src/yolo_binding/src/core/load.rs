use super::{YOLODevice, YOLO};
use core::panic;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::{
    collections::HashMap,
    // env,
    error::Error,
    // ffi::{c_char, CString},
    path::Path,
    // str::FromStr,
};
use tch::{vision, CModule, Device, Tensor};
// use winapi::um::libloaderapi::LoadLibraryA;

pub fn load_model_from_path(model_path: &str, cuda: bool) -> Result<YOLO, Box<dyn Error>> {
    //！ Load the model from the path
    //！ model_path: the path of the model
    //！ cuda: whether to use cuda
    //！ return: the YOLO model
    let device = if cuda == true {
        match Device::cuda_if_available() {
            Device::Cuda(device) => Device::Cuda(device),
            _ => panic!("No cuda device found"),
        }
    } else {
        Device::Cpu
    }; // device choiced
    // let device = if cuda == true {
    //     let mut libtorch_path = env::var("LIBTORCH").unwrap();
    //     libtorch_path.push_str(r"\lib\torch_cuda.dll");
    //     if Path::new(&libtorch_path).exists() {
    //         let path = CString::from_str(&libtorch_path).unwrap();
    //         unsafe {
    //             LoadLibraryA(path.as_ptr() as *const c_char);
    //         }
    //         Device::cuda_if_available()
    //     } else {
    //         panic!(
    //             "No {} exist,please check your libtorch version or set 'cuda' false instead",
    //             &libtorch_path
    //         );
    //     }
    // } else {
    //     Device::Cpu
    // }; // device choiced
    // println!("Module Device: {:?}", device);
    let model = CModule::load_on_device(Path::new(model_path), device).expect("load model failed");
    // println!("Model loaded");
    let device = match device {
        Device::Cuda(_) => YOLODevice::Gpu,
        Device::Cpu => YOLODevice::Gpu,
        _ => panic!("Other devices currently are not supported"),
    }; // model choiced

    let mut output_bytes: Vec<u8> = Vec::new();

    let file = File::open(Path::new(model_path))?;
    let mut reader = BufReader::new(file);

    let mut buffer = [0; 1];
    let mut record = false;
    let start_sequence = vec![0x5A; 9];
    let end_sequence = vec![0x50, 0x4B, 0x07, 0x08];
    let mut start_index = 0;
    let mut end_index = 0;

    while reader.read(&mut buffer)? > 0 {
        if record {
            output_bytes.write_all(&buffer)?;
        }

        if buffer[0] == start_sequence[start_index] {
            start_index += 1;
            if start_index == start_sequence.len() {
                record = true;
                start_index = 0;
            }
        } else {
            start_index = 0;
        }

        if buffer[0] == end_sequence[end_index] {
            end_index += 1;
            if end_index == end_sequence.len() {
                break;
            }
        } else {
            end_index = 0;
        }
    }

    let output_bytes = output_bytes[..output_bytes.len() - 4].to_vec();
    let v: Value = serde_json::from_slice(&output_bytes)?;
    let types = v["names"].as_object().unwrap();
    // println!("{:?} Types loaded", types.len());
    let mut names_map: HashMap<i64, String> = HashMap::new();
    for (key, value) in types.iter() {
        names_map.insert(
            key.trim().parse().unwrap(),
            value.as_str().unwrap().to_string(),
        );
    }

    Ok(YOLO {
        yolo_model: model,
        cuda: device,
        types: names_map,
    })
}

pub fn load_one_image(image_path: &str) -> Result<Tensor, Box<dyn Error>> {
    //！ Load one image from the path
    //！ image_path: the path of the image
    //！ return: the image tensor
    let image_tensor = vision::image::load(Path::new(&image_path))?;
    let image = tch::vision::image::resize(&image_tensor, 640, 640)
        .unwrap()
        .unsqueeze(0)
        .to_kind(tch::Kind::Float)
        / 255.;
    // println!("Resized Image size: {:?}", image);
    Ok(image)
}
pub fn load_images_from_dir(image_dir: &str) -> Result<Tensor, Box<dyn Error>> {
    //！ Load images from the directory
    //！ image_dir: the directory of the images
    //！ return: the images tensor
    let image_tensor = vision::image::load_dir(Path::new(&image_dir), 640, 640)
        .unwrap()
        .to_kind(tch::Kind::Float)
        / 255.;
    // println!("Resized Image size: {:?}", image_tensor);
    Ok(image_tensor)
}

pub fn load_one_image_from_memory(image_bytes: &[u8]) -> Result<Tensor, Box<dyn Error>> {
    //！ Load one image from the memory
    //！ image_bytes: the bytes of the image
    //！ return: the image tensor
    let image_tensor = vision::image::load_from_memory(image_bytes)?;
    let image = tch::vision::image::resize(&image_tensor, 640, 640)
        .unwrap()
        .unsqueeze(0)
        .to_kind(tch::Kind::Float)
        / 255.;
    // println!("Resized Image size: {:?}", image);
    Ok(image)
}
