use image::Rgb;
use nalgebra::{Matrix2x3, Matrix3, Matrix3x2, Matrix4, Vector2, Vector3, Vector4};

use crate::{
    consts::{LIGHT, LIGHT_DIR, WIDTH},
    img_io::WModel,
};

pub trait Shader {
    fn vertex(&mut self, f_idx: usize, v_idx: usize) -> Vector3<f32>;
    fn fragment(&mut self, color: &mut Rgb<u8>, bar: Vector3<f32>) -> bool;
}

pub struct GouphShader<'a> {
    varyng_uv: Matrix2x3<f32>,
    varyng_tri: Matrix3<f32>,
    coord_conv: Matrix4<f32>,
    model: &'a WModel,
    trans_nm: Matrix4<f32>,
    trans_shadow: Matrix4<f32>,
    trans_light: Matrix4<f32>,
    shadow_buf: Vec<f32>,
}

impl<'a> GouphShader<'a> {
    const K_D: f32 = 1.0;
    const K_S: f32 = 3.5;
}

impl<'a> GouphShader<'a> {
    pub fn new(
        coord_conv: Matrix4<f32>,
        model: &'a WModel,
        trans_nm: Matrix4<f32>,     // normal convert matrix
        trans_shadow: Matrix4<f32>, // shadow convert matrix
        trans_light: Matrix4<f32>,  // light convert matrix
        shadow_buf: Vec<f32>,
    ) -> Self {
        GouphShader {
            varyng_uv: Matrix2x3::zeros(),
            varyng_tri: Matrix3::zeros(),
            coord_conv,
            model,
            trans_nm,
            trans_shadow,
            trans_light,
            shadow_buf,
        }
    }

    fn convert_normal(&self, nm: &Vector3<f32>) -> Vector3<f32> {
        let nm_ = self.trans_nm * Vector4::new(nm.x, nm.y, nm.z, 1.0);
        Vector3::new(nm_.x, nm_.y, nm_.z)
    }
}

impl Shader for GouphShader<'_> {
    fn vertex(&mut self, f_idx: usize, v_idx: usize) -> Vector3<f32> {
        let uv = self.model.get_uv(f_idx, v_idx);
        self.varyng_uv[(0, v_idx)] = uv.x;
        self.varyng_uv[(1, v_idx)] = uv.y;
        let v = self.model.get_vertex(self.model.get_face(f_idx)[v_idx]);
        let v = self.coord_conv * Vector4::new(v.x, v.y, v.z, 1.0);
        let v = Vector3::new(v.x / v.w, v.y / v.w, v.z / v.w);
        self.varyng_tri[(0, v_idx)] = v.x;
        self.varyng_tri[(1, v_idx)] = v.y;
        self.varyng_tri[(2, v_idx)] = v.z;
        v
    }

    fn fragment(&mut self, color: &mut Rgb<u8>, bar: Vector3<f32>) -> bool {
        let uv = self.varyng_uv * bar;
        let p = self.varyng_tri * bar;

        let normal = self.model.get_normal_tex(uv);
        let normal = self.convert_normal(&normal);

        let shadow_p = self.trans_shadow * Vector4::new(p.x, p.y, p.z, 1.0);
        let shadow_p = Vector3::new(
            shadow_p.x / shadow_p.w,
            shadow_p.y / shadow_p.w,
            shadow_p.z / shadow_p.w,
        );
        let shadow_idx = shadow_p.x as usize + shadow_p.y as usize * WIDTH as usize;
        let shadow_intensity =
            0.3 + 0.7 * (self.shadow_buf[shadow_idx] < shadow_p.z + 0.01) as u8 as f32;

        let l = self.trans_light * Vector4::new(LIGHT_DIR.x, LIGHT_DIR.y, LIGHT_DIR.z, 0.0);
        let l = Vector3::new(l.x, l.y, l.z).normalize();
        let r = (2.0 * normal.dot(&l) * normal - l).normalize();

        let intensity = normal.dot(&l);

        let spec_color = self.model.specular_tex.get_pixel(
            (uv.x * self.model.specular_tex.width() as f32) as u32,
            (uv.y * self.model.specular_tex.height() as f32) as u32,
        );
        let specular_i = f32::max(r.z, 0.0).powf(5.0 + spec_color[0] as f32);

        for i in 0..3 {
            color[i] = (10.0
                + (color[i] as f32)
                    * shadow_intensity
                    * (GouphShader::K_D * intensity + (GouphShader::K_S * specular_i)))
                .min(255.0) as u8;
        }
        false
    }
}

pub struct DepthShader<'a> {
    coord_conv: Matrix4<f32>,
    model: &'a WModel,
    varying_tri: Matrix3<f32>,
}

impl<'a> DepthShader<'a> {
    pub fn new(coord_conv: Matrix4<f32>, model: &'a WModel) -> Self {
        DepthShader {
            coord_conv,
            model,
            varying_tri: Matrix3::identity(),
        }
    }
}

impl Shader for DepthShader<'_> {
    fn vertex(&mut self, f_idx: usize, v_idx: usize) -> Vector3<f32> {
        let v = self.model.get_vertex(self.model.get_face(f_idx)[v_idx]);
        let v = self.coord_conv * Vector4::new(v.x, v.y, v.z, 1.0);
        let v = Vector3::new(v.x / v.w, v.y / v.w, v.z / v.w);
        self.varying_tri[(0, v_idx)] = v.x;
        self.varying_tri[(1, v_idx)] = v.y;
        self.varying_tri[(2, v_idx)] = v.z;
        v
    }

    fn fragment(&mut self, color: &mut Rgb<u8>, bar: Vector3<f32>) -> bool {
        false
    }
}
