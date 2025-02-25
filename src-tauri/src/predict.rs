use std::ops::Index;

use yolo_binding::core::*;

pub async fn predict_image(model:&YOLO) -> Result<(i64,i64,i64), String> {
    let image_path = std::env::var("TEMP_DIR").unwrap() + "/capture.png";

    let image = load_one_image(&image_path).unwrap();
    let output = model.predict(&image).unwrap();

    let results = get_results(&output, 0.5, 0.8).unwrap();
    if results[0].len() == 0 {
        println!("No object detected");
        return Err("No object detected".to_string());
    }
    let (x0,y0,_,_,class,_)=results.index(0).index(0);
    let classes = model.types.clone();
    println!("x0: {}, y0: {}, class: {}", x0, y0, classes.get(&class).unwrap().to_string());
    Ok((*x0,*y0,*class))
}

pub fn load_model() -> YOLO {
    let model_path = std::env::var("MODEL_PATH").unwrap();
    let model = YOLO::new(&model_path, false);
    model
}