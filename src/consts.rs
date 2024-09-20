use lazy_static::lazy_static;
use nalgebra::Vector3;

pub static DEPTH: i32 = 100;
pub static CAMERA: Vector3<f32> = Vector3::new(1.0, 1.0, 3.0);
pub static FOVY: f32 = std::f32::consts::FRAC_PI_4;
pub static ASPECT: f32 = 1.0;

pub static AFRICAN_HEAD_OBJ: &str = "obj/african_head.obj";
pub static AFRICAN_HEAD_DIFFUSE: &str = "obj/african_head_diffuse.tga";
pub static AFRICAN_HEAD_SPECULAR: &str = "obj/african_head_spec.tga";
pub static AFRICAN_HEAD_NORMAL: &str = "obj/african_head_nm.tga";

pub static DIABLO3_OBJ: &str = "obj/diablo3_pose.obj";
pub static DIABLO3_DIFFUSE: &str = "obj/diablo3_pose_diffuse.tga";
pub static DIABLO3_SPECTURE: &str = "obj/diablo3_pose_spec.tga";

lazy_static! {
    pub static ref LIGHT_DIR: Vector3<f32> = Vector3::new(0.0, 1.0, 1.0).normalize();
}
