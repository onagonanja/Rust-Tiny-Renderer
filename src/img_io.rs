use image::imageops::flip_vertical_in_place;
use image::{ImageBuffer, ImageReader, Rgb};
use lazy_static::lazy_static;
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
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

pub fn load_image(
    filename: &str,
) -> Result<ImageBuffer<image::Rgb<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let mut img = ImageReader::open(filename)?.decode()?.to_rgb8();
    flip_vertical_in_place(&mut img);
    Ok(img)
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

pub struct WModel {
    pub face_num: usize,
    pub model: tobj::Model,
    pub texture: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub tex_uv: Vec<[Vector2<f32>; 3]>, // tex_uv[face_index] = [[u1, v1], [u2, v2], [u3, v3]]
    pub faces: Vec<Vector3<usize>>, // faces[face_index] = [vertex_index1, vertex_index2, vertex_index3]
    pub normals: Vec<[Vector3<f32>; 3]>, // normals[face_index] = [[nx1, ny1, nz1], [nx2, ny2, nz2], [nx3, ny3, nz3]]
}

impl WModel {
    pub fn new(model: tobj::Model, texture: ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Self {
        let face_num = model.mesh.indices.len() / 3;
        let mut tex_uv = vec![[Vector2::new(0.0, 0.0); 3]; face_num];
        let mut faces = vec![Vector3::new(0, 0, 0); face_num];
        let mut normals = vec![[Vector3::new(0.0, 0.0, 0.0); 3]; face_num];

        for i in 0..face_num {
            faces[i] = Vector3::new(
                model.mesh.indices[i * 3] as usize,
                model.mesh.indices[i * 3 + 1] as usize,
                model.mesh.indices[i * 3 + 2] as usize,
            );
            for j in 0..3 {
                tex_uv[i][j] = Vector2::new(
                    model.mesh.texcoords[2 * model.mesh.texcoord_indices[3 * i + j] as usize],
                    model.mesh.texcoords[2 * model.mesh.texcoord_indices[3 * i + j] as usize + 1],
                );
                normals[i][j] = Vector3::new(
                    model.mesh.normals[3 * model.mesh.normal_indices[3 * i + j] as usize],
                    model.mesh.normals[3 * model.mesh.normal_indices[3 * i + j] as usize + 1],
                    model.mesh.normals[3 * model.mesh.normal_indices[3 * i + j] as usize + 2],
                );
            }
        }

        WModel {
            face_num,
            model,
            texture,
            tex_uv,
            faces,
            normals,
        }
    }

    pub fn get_face(&self, face_index: usize) -> Vector3<usize> {
        self.faces[face_index]
    }

    pub fn get_vertex(&self, vertex_index: usize) -> Vector3<f32> {
        Vector3::new(
            self.model.mesh.positions[vertex_index * 3],
            self.model.mesh.positions[vertex_index * 3 + 1],
            self.model.mesh.positions[vertex_index * 3 + 2],
        )
    }

    pub fn get_face_uv(&self, face_index: usize) -> [Vector2<f32>; 3] {
        self.tex_uv[face_index]
    }

    pub fn get_face_normal(&self, face_index: usize) -> [Vector3<f32>; 3] {
        self.normals[face_index]
    }

    pub fn trans_normals(&mut self, m: &Matrix4<f32>) {
        let u: Matrix4<f32> = m.transpose().try_inverse().unwrap();
        for f in self.normals.iter_mut() {
            for n in f.iter_mut() {
                let mut n_ = Vector4::new(n.x, n.y, n.z, 0.0);
                n_ = u * n_;
                *n = Vector3::new(n_.x, n_.y, n_.z);
            }
        }
    }
}
