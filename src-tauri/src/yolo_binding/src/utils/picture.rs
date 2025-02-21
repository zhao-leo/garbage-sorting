use image::{DynamicImage, GenericImageView, ImageReader, Rgba};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use std::{collections::HashMap, error::Error, fs, ops::Index, path::Path};

fn export(
    tags: &HashMap<i64, String>,
    image: Vec<(i64, i64, i64, i64, i64, f64)>,
    mut picture: DynamicImage,
) -> Result<DynamicImage, Box<dyn Error>> {
    for (x, y, w, h, type_id, _) in image {
        let type_name = tags.get(&type_id).unwrap();
        let (x, y, w, h) = (x as f64, y as f64, w as f64, h as f64);
        let (width, height) = picture.dimensions();
        let (x1, y1, x2, y2) = (
            (x - w / 2.) / 640. * width as f64,
            (y - h / 2.) / 640. * height as f64,
            (x + w / 2.) / 640. * width as f64,
            (y + h / 2.) / 640. * height as f64,
        );
        let (x1, y1, x2, y2) = (x1 as f32, y1 as f32, x2 as f32, y2 as f32);
        draw_line_segment_mut(&mut picture, (x1, y1), (x2, y1), Rgba([255, 0, 0, 255]));
        draw_line_segment_mut(&mut picture, (x2, y1), (x2, y2), Rgba([255, 0, 0, 255]));
        draw_line_segment_mut(&mut picture, (x2, y2), (x1, y2), Rgba([255, 0, 0, 255]));
        draw_line_segment_mut(&mut picture, (x1, y2), (x1, y1), Rgba([255, 0, 0, 255]));
        let font = ab_glyph::FontRef::try_from_slice(include_bytes!("HarmonyOS_Sans_Regular.ttf")).unwrap();
        let scale = ab_glyph::PxScale::from(20.0);
        draw_text_mut(
            &mut picture,
            Rgba([255, 0, 0, 255]),
            x1 as i32,
            y1 as i32,
            scale,
            &font,
            type_name,
        );
    }
    Ok(picture)
}
pub fn export_images(
    tags: &HashMap<i64, String>,
    image: Vec<Vec<(i64, i64, i64, i64, i64, f64)>>,
    image_dir: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    //! Export the images with the bounding boxes
    //! tags: the tags of the classes
    //! image: the images with the bounding boxes
    //! image_dir: the directory of the images
    //! output_dir: the directory of the output images
    //! return: the result of the export
    let output_dir = Path::new(output_dir);
    if !output_dir.exists() {
        fs::create_dir(&Path::new(output_dir))?;
    }
    for (index, entry) in fs::read_dir(image_dir)?.enumerate() {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let image_reader = ImageReader::open(Path::new(&path))?
                .with_guessed_format()
                .unwrap();
            let img = image_reader.decode().unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let output_img = export(tags, image.index(index).to_vec(), img)?;
            let output_path = Path::new(output_dir).join(file_name);
            output_img.save(output_path)?;
        }
    }
    Ok(())
}

pub fn export_one_image(
    tags: &HashMap<i64, String>,
    image: Vec<(i64, i64, i64, i64, i64, f64)>,
    image_path: &str,
) -> Result<DynamicImage, Box<dyn Error>> {
    //! Export one image with the bounding boxes
    //! tags: the tags of the classes
    //! image: the image with the bounding boxes
    //! image_path: the path of the image
    //! return: the result of the export
    let image_reader = ImageReader::open(Path::new(image_path))?
        .with_guessed_format()
        .unwrap();
    let picture = image_reader.decode().unwrap();
    export(tags, image, picture)
}
