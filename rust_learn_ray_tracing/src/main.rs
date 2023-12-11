extern crate rust_learn_ray_tracing;

use rust_learn_ray_tracing::rt_lib::{scene::Scene, cam::Camera, vector3::Vector3, sphere::Sphere};
use rust_tiny_img::bmp::{self, bit_depth::BitDepth};

fn main(){
    let mut s = Scene::new();
    s.add_obj(Box::new(Sphere::new(&Vector3::new(0.0, 0.0, -10.0), 1.0)));
    s.add_obj(Box::new(Sphere::new(&Vector3::new(0.0, -201.0, -10.0), 200.0)));

    let c = Camera::new(
        800u32,
        600u32,
        Vector3::new(0f64, 0f64, 0f64),
        Vector3::new(0f64,0f64,-1f64),
        Vector3::new(1f64,0f64,0f64),
        0.1f64,
        45.0,
        16,
    );

    let start = std::time::Instant::now();
    let img = c.render(s,50);
    let duration = start.elapsed();
    println!("Time cost in render() is: {:?}", duration);

    match bmp::save_as_file(&img, "./test_out/test09.bmp",BitDepth::AllColors) {
        Err(msg) => {
            panic!("{}",msg);
        },
        _ => (),
    }
}