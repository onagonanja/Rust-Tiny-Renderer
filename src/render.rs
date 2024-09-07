use image::{ImageBuffer, Rgb};
use std::ops::{BitXor, Sub};
use tobj::Model;

#[derive(Clone, Copy)]
pub struct Pos2i {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
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

fn barycentric(pts: &[Pos2i; 3], p: Pos2i) -> Pos3f {
    let u = Pos3f {
        x: (pts[2].x - pts[0].x) as f32,
        y: (pts[1].x - pts[0].x) as f32,
        z: (pts[0].x - p.x) as f32,
    };
    let v = Pos3f {
        x: (pts[2].y - pts[0].y) as f32,
        y: (pts[1].y - pts[0].y) as f32,
        z: (pts[0].y - p.y) as f32,
    };

    let cross = u ^ v;

    if cross.z.abs() < 1.0 {
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

pub fn triangle(pts: &[Pos2i; 3], image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: &Rgb<u8>) {
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
        bboxmin.x = std::cmp::max(0, bboxmin.x.min(p.x));
        bboxmin.y = std::cmp::max(0, bboxmin.y.min(p.y));
        bboxmax.x = std::cmp::min(clamp.x, bboxmax.x.max(p.x));
        bboxmax.y = std::cmp::min(clamp.y, bboxmax.y.max(p.y));
    }
    for x in bboxmin.x..bboxmax.x {
        for y in bboxmin.y..bboxmax.y {
            let bc_screen = barycentric(pts, Pos2i { x, y });
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.put_pixel(x as u32, y as u32, *color);
        }
    }
}

pub fn render_obj(model: &Model, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for i in 0..model.mesh.indices.len() / 3 {
        let face: [usize; 3] = [
            model.mesh.indices[3 * i] as usize,
            model.mesh.indices[3 * i + 1] as usize,
            model.mesh.indices[3 * i + 2] as usize,
        ];

        let pts = [
            Pos2i {
                x: ((model.mesh.positions[3 * face[0]] + 1.0) * image.width() as f32 / 2.0) as i32,
                y: ((model.mesh.positions[3 * face[0] + 1] + 1.0) * image.height() as f32 / 2.0)
                    as i32,
            },
            Pos2i {
                x: ((model.mesh.positions[3 * face[1]] + 1.0) * image.width() as f32 / 2.0) as i32,
                y: ((model.mesh.positions[3 * face[1] + 1] + 1.0) * image.height() as f32 / 2.0)
                    as i32,
            },
            Pos2i {
                x: ((model.mesh.positions[3 * face[2]] + 1.0) * image.width() as f32 / 2.0) as i32,
                y: ((model.mesh.positions[3 * face[2] + 1] + 1.0) * image.height() as f32 / 2.0)
                    as i32,
            },
        ];

        let world_coords = get_world_coords(model, &face);

        let _n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);
        let n = _n.normalize();

        let light_dir = Pos3f {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };

        let intensity = n.dot(light_dir);
        //println!("{:?}", intensity);

        if intensity <= 0.0 {
            continue;
        }

        let color = Rgb([((intensity * 255.0) as u8).clamp(0, 255); 3]);
        //println!("{:?}", color);

        triangle(&pts, image, &color);
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
