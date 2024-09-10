mod consts;
mod geometry;
mod img_io;
mod render;

use image::{ImageBuffer, Rgb};
use std::time;

use img_io::*;
use render::*;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 800;

fn main() {
    let now = time::Instant::now();

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = init_image(WIDTH, HEIGHT);

    let model = img_io::load_obj("obj/african_head.obj");

    let texture = img_io::load_image("obj/african_head_diffuse.tga").unwrap();
    //let texture = img_io::load_image("obj/uvsample5.png").unwrap();
    //render_obj_line(&model, &mut image);
    render_obj(&model, &mut image, &texture);

    img_io::output_image("output.png", &mut image);
    println!("{:?}", now.elapsed());
}
