mod consts;
mod geometry;
mod img_io;
mod render;
mod shader;

use image::{ImageBuffer, Rgb};
use std::time;

use consts::*;
use img_io::*;
use render::*;

static WIDTH: u32 = 1600;
static HEIGHT: u32 = 1600;

fn main() {
    let now = time::Instant::now();
    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = init_image(WIDTH, HEIGHT);
    let model = img_io::load_obj(AFRICAN_HEAD_OBJ);
    let texture = img_io::load_image(AFRICAN_HEAD_DIFFUSE).unwrap();
    let specture_tex = img_io::load_image(AFRICAN_HEAD_SPECULAR).unwrap();
    let normal_tex = img_io::load_image(AFRICAN_HEAD_NORMAL).unwrap();
    // let model = img_io::load_obj(DIABLO3_OBJ);
    // let texture = img_io::load_image(DIABLO3_DIFFUSE).unwrap();
    // let specture_tex = img_io::load_image(DIABLO3_SPECTURE).unwrap();
    // let normal_tex = img_io::load_image(DIABLO3_NORMAL).unwrap();
    let mut model = WModel::new(model, texture, specture_tex, normal_tex);
    render_obj(&mut model, &mut image);
    img_io::output_image("output.png", &mut image);
    println!("{:?}", now.elapsed());
}
