extern crate rust_tiny_img;
use rust_tiny_img::{img::Img, color::Color, bmp::{self, bit_depth::BitDepth}};

pub mod vector3;
pub mod geometry;
pub mod ray;
pub mod scene;
pub mod cam;
pub mod sphere;

use crate::rt_lib::{vector3::Vector3, ray::Ray, sphere::Sphere};

use self::{scene::Scene, cam::Camera};

///render scene with camera
pub fn render(s : &Scene, c : &Camera){
    let sw = c.get_w();
    let sh = c.get_h();

    let mut img = Img::new(sw, sh);

    println!("start render");

    let test_sphere = Sphere::new(&Vector3::new(0.0, 0.0, -10.0), 1.0);
    let start_pos = c.get_piexl_view_pos_start_pos();
    for w in 0..sw {
        println!("remaining:{} col",sw - w);
        for h in 0..sh {
            let dir = c.get_piexl_view_pos(start_pos, w, h);
            let ray = Ray::new(c.get_pos(),dir - c.get_pos());
            let color = ray.get_color(&test_sphere);
            match img.set_pixel(w, h, color) {
                Err(msg) => {
                    panic!("{}",msg);
                },
                _ => (),
            }
        }
    }

    println!("Success Done");

    let save_result = bmp::save_as_file(&img, "./test_out/test01.bmp",BitDepth::AllColors);
    match save_result {
        Err(msg) => {
            panic!("{}",msg);
        },
        _ => (),
    }
}