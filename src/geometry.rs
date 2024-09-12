use crate::consts::DEPTH;
use nalgebra::{Matrix4, Vector3};

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

pub fn get_lookat(eye: Vector3<f32>, center: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let z = (eye - center).normalize();
    let x = up.cross(&z).normalize();
    let y = z.cross(&x).normalize();

    let mut minv: Matrix4<f32> = Matrix4::identity();
    let mut tr: Matrix4<f32> = Matrix4::identity();

    for i in 0..3 {
        minv[(0, i)] = x[i];
        minv[(1, i)] = y[i];
        minv[(2, i)] = z[i];
        tr[(i, 3)] = -eye[i];
    }

    minv * tr
}

pub fn get_projection(fovy: f32, aspect: f32, n: f32) -> Matrix4<f32> {
    let f = 1.0 / (fovy / 2.0).tan();
    let mut m: Matrix4<f32> = Matrix4::zeros();

    m[(0, 0)] = f / aspect;
    m[(1, 1)] = f;
    m[(2, 2)] = (f + n) / (f - n);
    m[(2, 3)] = 2.0 * f * n / (n - f);
    m[(3, 2)] = -1.0;

    m
}
