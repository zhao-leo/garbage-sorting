use yolo_binding::core::*;

pub async fn predict_image() -> Result<(i64,i64,String), String> {
    let image_path = std::env::var("TEMP_DIR").unwrap() + "/capture.png";
    let model_path = std::env::var("MODEL_PATH").unwrap();
    let model = YOLO::new(&model_path, false);
    
    let image = load_one_image(&image_path).unwrap();
    let output = model.predict(&image).unwrap();

    let results = get_results(&output, 0.5, 0.8).unwrap();
    let (x0,y0,_,_,class,_)=results[0][0];
    let classes = model.types.clone();
    let class = classes.get(&class).unwrap().to_string();
    Ok((x0,y0,class))
}