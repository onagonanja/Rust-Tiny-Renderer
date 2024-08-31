use image::{ImageBuffer, Rgb};
use tobj::Model;

pub fn line(_x0: i32, _y0: i32, _x1: i32, _y1: i32, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let (mut x0, mut x1, mut y0, mut y1) = (_x0, _x1, _y0, _y1);
    let mut steep = false;

    if (x0 - x1).abs() < (y0 - y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = dy.abs() * 2;
    let mut error = 0;

    let mut y = y0;

    for x in x0..x1 {
        if steep {
            image.put_pixel(y as u32, x as u32, Rgb([255, 255, 255]));
        } else {
            image.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
        }

        error += derror;
        if error > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

pub fn render_obj(model: &Model, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for i in 0..model.mesh.indices.len() / 3 {
        let face = [
            model.mesh.indices[3 * i],
            model.mesh.indices[3 * i + 1],
            model.mesh.indices[3 * i + 2],
        ];

        let verticles = [
            (
                model.mesh.positions[3 * face[0] as usize],
                model.mesh.positions[3 * face[0] as usize + 1],
                model.mesh.positions[3 * face[0] as usize + 2],
            ),
            (
                model.mesh.positions[3 * face[1] as usize],
                model.mesh.positions[3 * face[1] as usize + 1],
                model.mesh.positions[3 * face[1] as usize + 2],
            ),
            (
                model.mesh.positions[3 * face[2] as usize],
                model.mesh.positions[3 * face[2] as usize + 1],
                model.mesh.positions[3 * face[2] as usize + 2],
            ),
        ];

        for j in 0..3 {
            let v0 = verticles[j];
            let v1 = verticles[(j + 1) % 3];

            let x0 = ((v0.0 + 1.0) * image.width() as f32 / 2.0) as i32;
            let y0 = ((v0.1 + 1.0) * image.height() as f32 / 2.0) as i32;
            let x1 = ((v1.0 + 1.0) * image.width() as f32 / 2.0) as i32;
            let y1 = ((v1.1 + 1.0) * image.height() as f32 / 2.0) as i32;

            if x0 >= image.width() as i32
                || x0 < 0
                || y0 >= image.height() as i32
                || y0 < 0
                || x1 >= image.width() as i32
                || x1 < 0
                || y1 >= image.height() as i32
                || y1 < 0
            {
                continue;
            }

            line(x0, y0, x1, y1, image);
        }
    }
}
