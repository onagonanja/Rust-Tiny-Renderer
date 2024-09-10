use crate::consts::DEPTH;
use nalgebra::Matrix4;

pub fn get_viewport(x: f32, y: f32, w: f32, h: f32) -> Matrix4<f32> {
    let mut m: Matrix4<f32> = Matrix4::identity();

    m[(0, 3)] = x + w / 2.0;
    m[(1, 3)] = y + h / 2.0;
    m[(2, 3)] = 0.5;

    m[(0, 0)] = w / 2.0;
    m[(1, 1)] = h / 2.0;
    m[(2, 2)] = DEPTH as f32 / 2.0;

    m
}
