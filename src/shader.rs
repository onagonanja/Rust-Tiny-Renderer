// use nalgebra::{Matrix4, Vector3};
// use tobj::Model;

// struct GouphShader {
//     varyng_intensity: Vec<f32>,
//     coord_conv: Matrix4<f32>,
// }

// impl GouphShader {
//     fn new(v_num: usize, coord_conv: Matrix4<f32>) -> Self {
//         GouphShader {
//             varyng_intensity: vec![0.0; v_num],
//             coord_conv,
//         }
//     }

//     fn vertex(&mut self, f_idx: usize, v_idx: usize, model: &Model) -> Vector3<f32> {
//         self.varyng_intensity[v_idx] = 0.0.max(model.mesh.material.diffuse[0]);
//     }

//     fn fragment(&mut self, intensity: Vector3<f32>) -> Vector3<f32> {
//         self.varyng_intensity = intensity;
//         intensity
//     }
// }
