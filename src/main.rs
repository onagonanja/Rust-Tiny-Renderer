mod consts;
mod geometry;
mod img_io;
mod render;
mod shader;

use consts::{DIABLO3_DIFFUSE, DIABLO3_OBJ};
use image::{ImageBuffer, Rgb};
use std::time;

use img_io::*;
use render::*;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 800;

fn main() {
    let now = time::Instant::now();
    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = init_image(WIDTH, HEIGHT);
    let model = img_io::load_obj(DIABLO3_OBJ);
    let texture = img_io::load_image(DIABLO3_DIFFUSE).unwrap();
    let model = WModel::new(model, texture);
    render_obj(&model, &mut image);
    img_io::output_image("output.png", &mut image);
    println!("{:?}", now.elapsed());
}
