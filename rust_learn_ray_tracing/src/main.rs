extern crate rust_learn_ray_tracing;

use std::sync::Arc;

use rust_learn_ray_tracing::rt_lib::{scene::Scene, cam::Camera, vector3::Vector3, sphere::Sphere, material::{mat_lambertian::Lambertian, mat_metal::Metal}};
use rust_tiny_img::bmp::{self, bit_depth::BitDepth};

fn main(){
    let mut s = Scene::new();

    let mat_ground = Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Vector3::new(0.7, 0.3, 0.3)));
    let mat_left = Arc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8),0.3));
    let mat_right = Arc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2),1.0));

    s.add_obj(Box::new(Sphere::new(&Vector3::new(0.0, -201.0, -10.0), 200.0,mat_ground))); 
    s.add_obj(Box::new(Sphere::new(&Vector3::new(0.0, 0.0, -10.0), 1.0, mat_center)));
    s.add_obj(Box::new(Sphere::new(&Vector3::new(-2.0, 0.0, -10.0), 1.0,mat_left)));
    s.add_obj(Box::new(Sphere::new(&Vector3::new(2.0, 0.0, -10.0), 1.0,mat_right)));

    let c = Camera::new(
        800u32,
        600u32,
        Vector3::new(0f64, 0f64, 0f64),
        Vector3::new(0f64,0f64,-1f64),
        Vector3::new(1f64,0f64,0f64),
        0.1f64,
        60.0,
        16,
    );

    let start = std::time::Instant::now();
    let mut img = c.render(s,50);
    let duration = start.elapsed();
    println!("Time cost in render() is: {:?}", duration);
    img.liner_to_gamma();

    match bmp::save_as_file(&img, "./test_out/test010-3.bmp",BitDepth::AllColors) {
        Err(msg) => {
            panic!("{}",msg);
        },
        _ => (),
    }
}