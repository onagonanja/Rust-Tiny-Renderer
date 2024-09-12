use nalgebra::{Matrix4, Vector3};
use tobj::Model;

struct GouphShader {
    varyng_intensity: Vector3<f32>,
}

impl GouphShader {
    fn new() -> Self {
        GouphShader {
            varyng_intensity: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    // fn vertex(
    //     &mut self,
    //     f_idx: i32,
    //     v_idx: i32,
    //     cor_conv: &Matrix4<f32>,
    //     model: &Model,
    // ) -> Vector3<f32> {

    // }

    // fn fragment(&mut self, intensity: Vector3<f32>) -> Vector3<f32> {
    //     self.varyng_intensity = intensity;
    //     intensity
    // }
}
