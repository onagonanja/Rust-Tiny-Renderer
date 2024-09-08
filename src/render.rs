use image::{ImageBuffer, Rgb};
use std::{
    ops::{BitXor, Sub},
    vec,
};
use tobj::Model;

#[derive(Clone, Copy, Debug)]
pub struct Pos2i {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Pos2f {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Pos3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Pos3f {
    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Pos3f {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Sub for Pos3f {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Pos3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// calculate cross product
impl BitXor for Pos3f {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Pos3f {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

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

fn barycentric(pts: &[Pos3f; 3], p: Pos2i) -> Pos3f {
    let u = Pos3f {
        x: pts[2].x - pts[0].x,
        y: pts[1].x - pts[0].x,
        z: pts[0].x - p.x as f32,
    };
    let v = Pos3f {
        x: pts[2].y - pts[0].y,
        y: pts[1].y - pts[0].y,
        z: pts[0].y - p.y as f32,
    };
    let cross = u ^ v;
    if cross.z.abs() == 0.0 {
        return Pos3f {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        };
    }
    Pos3f {
        x: 1.0 - (cross.x + cross.y) / cross.z,
        y: cross.y / cross.z,
        z: cross.x / cross.z,
    }
}

fn get_world_coords(model: &Model, face: &[usize; 3]) -> [Pos3f; 3] {
    let mut world_coords = [Pos3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; 3];
    for i in 0..3 {
        world_coords[i] = Pos3f {
            x: model.mesh.positions[3 * face[i]],
            y: model.mesh.positions[3 * face[i] + 1],
            z: model.mesh.positions[3 * face[i] + 2],
        };
    }
    world_coords
}

fn get_tringle_positions(model: &Model, face: &[usize; 3], width: i32, height: i32) -> [Pos3f; 3] {
    let mut positions = [Pos3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; 3];
    for i in 0..3 {
        positions[i] = Pos3f {
            x: (model.mesh.positions[3 * face[i]] + 1.0) * width as f32 / 2.0,
            y: (model.mesh.positions[3 * face[i] + 1] + 1.0) * height as f32 / 2.0,
            z: (model.mesh.positions[3 * face[i] + 2] + 1.0),
        };
    }
    positions
}

fn get_triangle_tex_coords(tex_coords: &[f32], tex_index: &[u32], index: usize) -> [Pos3f; 3] {
    let mut triangle_tex_coords = [Pos3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; 3];
    for i in 0..3 {
        triangle_tex_coords[i] = Pos3f {
            x: tex_coords[2 * tex_index[3 * index + i] as usize],
            y: tex_coords[2 * tex_index[3 * index + i] as usize + 1],
            z: 0.0,
        };
    }
    triangle_tex_coords
}

pub fn render_triangle(
    pts: &[Pos3f; 3],
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    z_buffer: &mut [f32],
    tex_image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    triangle_tex_coords: &[Pos3f; 3],
    intensity: f32,
) {
    let mut bboxmin = Pos2i {
        x: (image.width() - 1) as i32,
        y: (image.height() - 1) as i32,
    };

    let mut bboxmax = Pos2i { x: 0, y: 0 };

    let clamp = Pos2i {
        x: (image.width() - 1) as i32,
        y: (image.height() - 1) as i32,
    };

    for p in pts.iter().take(3) {
        bboxmin.x = std::cmp::max(0, bboxmin.x.min((p.x + 1.0) as i32));
        bboxmin.y = std::cmp::max(0, bboxmin.y.min((p.y + 1.0) as i32));
        bboxmax.x = std::cmp::min(clamp.x, bboxmax.x.max((p.x + 1.0) as i32));
        bboxmax.y = std::cmp::min(clamp.y, bboxmax.y.max((p.y + 1.0) as i32));
    }

    let mut min_u: f32 = 1.0;
    let mut min_v: f32 = 1.0;

    let mut max_u: f32 = 0.0;
    let mut max_v: f32 = 0.0;

    for i in 0..3 {
        min_u = min_u.min(triangle_tex_coords[i].x);
        min_v = min_v.min(triangle_tex_coords[i].y);
        max_u = max_u.max(triangle_tex_coords[i].x);
        max_v = max_v.max(triangle_tex_coords[i].y);
    }

    for x in bboxmin.x..bboxmax.x {
        for y in bboxmin.y..bboxmax.y {
            let bc_screen = barycentric(pts, Pos2i { x, y });

            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            let z: f32 = pts[0].z * bc_screen.x + pts[1].z * bc_screen.y + pts[2].z * bc_screen.z;

            if z_buffer[(x + y * image.width() as i32) as usize] < z {
                z_buffer[(x + y * image.width() as i32) as usize] = z;
                let uv = Pos2f {
                    x: triangle_tex_coords[0].x * bc_screen.x
                        + triangle_tex_coords[1].x * bc_screen.y
                        + triangle_tex_coords[2].x * bc_screen.z,
                    y: triangle_tex_coords[0].y * bc_screen.x
                        + triangle_tex_coords[1].y * bc_screen.y
                        + triangle_tex_coords[2].y * bc_screen.z,
                };

                min_u = min_u.min(uv.x);
                min_v = min_v.min(uv.y);
                max_u = max_u.max(uv.x);
                max_v = max_v.max(uv.y);

                let color = tex_image.get_pixel(
                    (uv.x * tex_image.width() as f32) as u32,
                    (uv.y * tex_image.height() as f32) as u32,
                );
                let color = Rgb([
                    (color[0] as f32 * intensity) as u8,
                    (color[1] as f32 * intensity) as u8,
                    (color[2] as f32 * intensity) as u8,
                ]);
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn render_obj(
    model: &Model,
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    tex_image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let mut z_buffer = vec![f32::MIN; (image.width() * image.height()) as usize];
    for i in 0..model.mesh.indices.len() / 3 {
        let face: [usize; 3] = [
            model.mesh.indices[3 * i] as usize,
            model.mesh.indices[3 * i + 1] as usize,
            model.mesh.indices[3 * i + 2] as usize,
        ];

        let pts = get_tringle_positions(model, &face, image.width() as i32, image.height() as i32);

        let world_coords = get_world_coords(model, &face);

        let _n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);
        let n = _n.normalize();

        let light_dir = Pos3f {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };

        let intensity = n.dot(light_dir);

        if intensity <= 0.0 {
            continue;
        }

        let triangle_tex_coords: [Pos3f; 3] =
            get_triangle_tex_coords(&model.mesh.texcoords, &model.mesh.texcoord_indices, i);

        render_triangle(
            &pts,
            image,
            &mut z_buffer,
            tex_image,
            &triangle_tex_coords,
            intensity,
        );
    }
}

#[allow(dead_code)]
pub fn render_obj_line(model: &Model, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for i in 0..model.mesh.indices.len() / 3 {
        let face: [usize; 3] = [
            model.mesh.indices[3 * i] as usize,
            model.mesh.indices[3 * i + 1] as usize,
            model.mesh.indices[3 * i + 2] as usize,
        ];

        let verticles = [(
            model.mesh.positions[3 * face[0]],
            model.mesh.positions[3 * face[0] + 1],
            model.mesh.positions[3 * face[0] + 2],
            model.mesh.positions[3 * face[1]],
            model.mesh.positions[3 * face[1] + 1],
            model.mesh.positions[3 * face[1] + 2],
            model.mesh.positions[3 * face[2]],
            model.mesh.positions[3 * face[2] + 1],
            model.mesh.positions[3 * face[2] + 2],
        )];

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
