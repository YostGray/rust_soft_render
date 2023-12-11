extern crate rust_learn_ray_tracing;

use rust_learn_ray_tracing::rt_lib::{scene::Scene, cam::Camera, vector3::Vector3, render};

fn main(){
    let s = Scene{};
    let c = Camera::new(
        1920u32,
        1080u32,
        Vector3::new(0f64, 0f64, 0f64),
        Vector3::new(0f64,0f64,-1f64),
        Vector3::new(1f64,0f64,0f64),
        0.1f64,
        45.0);
    render(&s, &c);
}