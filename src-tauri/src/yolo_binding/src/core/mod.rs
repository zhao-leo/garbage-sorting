mod export;
mod load;
mod predict;
use std::{collections::HashMap, error::Error};
use tch::{CModule, Tensor};
pub enum YOLODevice {
    Cpu,
    Gpu,
}
pub struct YOLO {
    yolo_model: CModule,
    cuda: YOLODevice,
    pub types: HashMap<i64, String>,
}

impl YOLO {
    pub fn new(model_path: &str, cuda_is_available: bool) -> Self {
        //! Create a new YOLO model
        //! model_path: the path of the model
        //! cuda_is_available: whether to use cuda
        load::load_model_from_path(model_path, cuda_is_available).unwrap()
    }

    pub fn predict(&self, input: &Tensor) -> Result<Tensor, Box<dyn Error>> {
        //! Predict the input tensor
        //! input: the input tensor
        //! return: the output tensor
        predict::pred(self, input)
    }
}
pub fn load_one_image(image_path: &str) -> Result<Tensor, Box<dyn Error>> {
    //! Load one image from the path
    //! image_path: the path of the image
    //! return: the image tensor
    let image = load::load_one_image(image_path)?;
    Ok(image)
}
pub fn load_images_from_dir(image_dir: &str) -> Result<Tensor, Box<dyn Error>> {
    //! Load images from the directory
    //! image_dir: the directory of the images
    //! return: the images tensor
    let images = load::load_images_from_dir(image_dir)?;
    Ok(images)
}
pub fn load_one_image_from_memory(image_bytes: &[u8]) -> Result<Tensor, Box<dyn Error>> {
    //! Load one image from the memory
    //! image_bytes: the bytes of the image
    //! return: the image tensor
    let image = load::load_one_image_from_memory(image_bytes)?;
    Ok(image)
}
pub fn get_results(
    input: &Tensor,
    confidence: f64,
    threshold: f64,
) -> Result<Vec<Vec<(i64, i64, i64, i64, i64, f64)>>, Box<dyn Error>> {
    //! Get the results from the output tensor
    //! input: the output tensor from the model [n, X, 8400]
    //! confidence: the confidence threshold
    //! threshold: the NMS threshold
    //! return: the results of the bounding boxes
    //! x, y, w, h, class, confidence
    let mut result = Vec::new();
    let (batch_size, _, _) = input.size3().unwrap();
    for i in 0..batch_size {
        let res = export::post_process(&input.get(i), confidence, threshold)?;
        result.push(res);
    }
    Ok(result)
}
