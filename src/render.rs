use image::{ImageBuffer, Rgb};
use nalgebra::{Vector2, Vector3};

use crate::{
    consts::{ASPECT, CAMERA, FOVY},
    geometry,
    img_io::WModel,
    shader::GouphShader,
};

// calculate barycentric coordinates
fn barycentric(pts: &[Vector3<f32>; 3], p: Vector2<f32>) -> Vector3<f32> {
    let u = Vector3::new(pts[2].x - pts[0].x, pts[1].x - pts[0].x, pts[0].x - p.x);
    let v = Vector3::new(pts[2].y - pts[0].y, pts[1].y - pts[0].y, pts[0].y - p.y);
    let cross = u.cross(&v);
    if cross.z.abs() < 1.0 {
        return Vector3::new(-1.0, 1.0, 1.0);
    }
    Vector3::new(
        1.0 - (cross.x + cross.y) / cross.z,
        cross.y / cross.z,
        cross.x / cross.z,
    )
}

pub fn triangle(
    model: &WModel,
    face_index: usize,
    pts: &[Vector3<f32>; 3],
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    z_buffer: &mut [f32],
    shader: &mut GouphShader,
) {
    let triangle_tex_coords = model.get_face_uv(face_index);

    let mut bboxmin = Vector2::new((image.width() - 1) as i32, (image.height() - 1) as i32);
    let mut bboxmax = Vector2::new(0, 0);
    let clamp = Vector2::new(image.width() as i32 - 1, image.height() as i32 - 1);

    for p in pts.iter().take(3) {
        bboxmin.x = std::cmp::max(0, bboxmin.x.min((p.x + 1.0) as i32));
        bboxmin.y = std::cmp::max(0, bboxmin.y.min((p.y + 1.0) as i32));
        bboxmax.x = std::cmp::min(clamp.x, bboxmax.x.max((p.x + 1.0) as i32));
        bboxmax.y = std::cmp::min(clamp.y, bboxmax.y.max((p.y + 1.0) as i32));
    }

    for x in bboxmin.x..bboxmax.x {
        for y in bboxmin.y..bboxmax.y {
            let bc_screen = barycentric(pts, Vector2::new(x as f32, y as f32));

            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            let z: f32 = pts[0].z * bc_screen.x + pts[1].z * bc_screen.y + pts[2].z * bc_screen.z;

            if z_buffer[(x + y * image.width() as i32) as usize] < z {
                z_buffer[(x + y * image.width() as i32) as usize] = z;
                let uv = Vector2::new(
                    triangle_tex_coords[0].x * bc_screen.x
                        + triangle_tex_coords[1].x * bc_screen.y
                        + triangle_tex_coords[2].x * bc_screen.z,
                    triangle_tex_coords[0].y * bc_screen.x
                        + triangle_tex_coords[1].y * bc_screen.y
                        + triangle_tex_coords[2].y * bc_screen.z,
                );

                let color = model.texture.get_pixel(
                    (uv.x * model.texture.width() as f32) as u32,
                    (uv.y * model.texture.height() as f32) as u32,
                );
                let mut color = Rgb([color[0], color[1], color[2]]);
                shader.fragment(face_index, bc_screen, &mut color);
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn render_obj(model: &mut WModel, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut z_buffer = vec![f32::MIN; (image.width() * image.height()) as usize];

    let projection = geometry::get_projection(FOVY, ASPECT, -1.0);
    let viewport = geometry::get_viewport(
        image.width() as f32 / 8.0,
        image.height() as f32 / 8.0,
        image.width() as f32 * 3.0 / 4.0,
        image.height() as f32 * 3.0 / 4.0,
    );
    let lookat = geometry::get_lookat(
        Vector3::new(CAMERA.x, CAMERA.y, CAMERA.z),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let cor_conv = viewport * projection * lookat;
    model.trans_normals(&cor_conv);
    let mut shader = GouphShader::new(model.face_num, cor_conv, &model);

    for i in 0..model.face_num {
        let mut screen_coords = [Vector3::new(0.0, 0.0, 0.0); 3];

        for j in 0..3 {
            screen_coords[j] = shader.vertex(i, j)
        }

        triangle(&model, i, &screen_coords, image, &mut z_buffer, &mut shader);
    }
}
