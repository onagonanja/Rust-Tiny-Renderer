use criterion::{criterion_group, criterion_main, Criterion};
use image::ImageBuffer;

use lib::img_io::*;
use lib::render::*;

pub fn renderer_benchmark(c: &mut Criterion) {
    let model = load_obj("obj/african_head.obj");

    // c.bench_function("line", |b| {
    //     b.iter(|| line(13, 20, 600, 400, &mut ImageBuffer::new(800, 600)))
    // });

    // c.bench_function("obj", |b| {
    //     b.iter(|| render_obj_line(&model, &mut ImageBuffer::new(800, 600)))
    // });
}

criterion_group!(benches, renderer_benchmark,);
criterion_main!(benches);
