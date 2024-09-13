use image::{Pixel, Rgb};
use nalgebra::{Matrix4, Vector3, Vector4};

use crate::{consts::LIGHT_DIR, img_io::WModel};

pub struct GouphShader<'a> {
    varyng_intensity: Vec<Vector3<f32>>, // varyng_intensity[face_index] = [intensity_v1, intensity_v2, intensity_v3]
    coord_conv: Matrix4<f32>,
    model: &'a WModel,
}

impl<'a> GouphShader<'a> {
    pub fn new(f_num: usize, coord_conv: Matrix4<f32>, model: &'a WModel) -> Self {
        GouphShader {
            varyng_intensity: vec![Vector3::new(1.0, 1.0, 1.0); f_num],
            coord_conv,
            model,
        }
    }

    pub fn vertex(&mut self, f_idx: usize, v_idx: usize) -> Vector3<f32> {
        let normals = self.model.get_face_normal(f_idx);
        self.varyng_intensity[f_idx][v_idx] = f32::max(normals[v_idx].dot(&LIGHT_DIR), 0.0);
        let v = self.model.get_vertex(self.model.get_face(f_idx)[v_idx]);
        let v = self.coord_conv * Vector4::new(v.x, v.y, v.z, 1.0);
        let v = Vector3::new(v.x / v.w, v.y / v.w, v.z / v.w);
        v
    }

    pub fn fragment(&mut self, f_idx: usize, bar: Vector3<f32>, color: &mut Rgb<u8>) -> bool {
        let intensity = self.varyng_intensity[f_idx].dot(&bar);
        //println!("intensity: {}", intensity);
        color.apply(|c| (c as f32 * intensity) as u8);
        false
    }
}
