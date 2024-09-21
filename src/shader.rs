use image::Rgb;
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};

use crate::img_io::WModel;

pub struct GouphShader<'a> {
    varyng_intensity: Vec<Vector3<f32>>, // varyng_intensity[face_index] = [intensity_v1, intensity_v2, intensity_v3] (for calc L・N in Phong model)
    coord_conv: Matrix4<f32>,
    model: &'a WModel,
}

impl<'a> GouphShader<'a> {
    const K_D: f32 = 1.0;
    const K_S: f32 = 1.5;
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
        self.varyng_intensity[f_idx][v_idx] = f32::max(normals[v_idx].dot(&self.model.light), 0.0);
        let v = self.model.get_vertex(self.model.get_face(f_idx)[v_idx]);
        let v = self.coord_conv * Vector4::new(v.x, v.y, v.z, 1.0);
        let v = Vector3::new(v.x / v.w, v.y / v.w, v.z / v.w);
        v
    }

    pub fn fragment(
        &mut self,
        color: &mut Rgb<u8>,
        spec_color: &Rgb<u8>,
        uv: &Vector2<f32>,
    ) -> bool {
        let normal = self.model.get_normal_tex(*uv);
        let intensity = normal.dot(&self.model.light);
        let l = self.model.light;
        let r = (2.0 * normal.dot(&l) * normal - l).normalize();
        let specular_i = f32::max(r.z, 0.0).powf(5.0 + spec_color[0] as f32);
        for i in 0..3 {
            color[i] = (10.0
                + (color[i] as f32)
                    * (GouphShader::K_D * intensity + (GouphShader::K_S * specular_i)))
                .min(255.0) as u8;
        }
        false
    }
}
