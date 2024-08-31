use image::imageops::flip_vertical_in_place;
use image::{ImageBuffer, Rgb};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::Path;

pub fn init_image(width: u32, height: u32) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, image::Rgb([0, 0, 0]))
        }
    }
    image
}

pub fn output_image(filename: &str, image: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    flip_vertical_in_place(image);
    image.save(filename).unwrap();
    println!("Saved image!");
}

pub fn load_obj(filename: &str) -> tobj::Model {
    let (model, _) = tobj::load_obj(Path::new(filename), &tobj::LoadOptions::default()).unwrap();
    model.first().unwrap().clone()
}

lazy_static! {
    pub static ref COLORS: HashMap<String, image::Rgb<u8>> = {
        let mut colors = HashMap::new();
        colors.insert("black".to_string(), Rgb([0, 0, 0]));
        colors.insert("white".to_string(), Rgb([255, 255, 255]));
        colors.insert("red".to_string(), Rgb([255, 0, 0]));
        colors.insert("green".to_string(), Rgb([0, 255, 0]));
        colors.insert("blue".to_string(), Rgb([0, 0, 255]));
        colors
    };
}
