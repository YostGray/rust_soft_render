use std::thread;

use rust_tiny_img::{img::Img, color::{Color, self}};
use rand::{Rng, thread_rng, rngs::ThreadRng};
use crate::rt_lib::ray::Ray;
use super::{vector3::Vector3, scene::Scene};

/// the view point to render scene.
/// use left hand coordinary.
pub struct Camera {
    screen_w : u32,
    screen_h : u32,

    pos : Vector3,
    dir : Vector3,
    dir_w : Vector3,
    dir_up : Vector3,

    near : f64,
    fov_w  : f64,//degree
    view_w : f64,
    view_h : f64,
    
    vw_step : f64,
    vh_step : f64,

    sample_pre_pixel:u64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
            screen_w: Default::default(), 
            screen_h: Default::default(), 
            pos: Default::default(), 
            dir: Default::default(), 
            dir_w: Default::default(), 
            dir_up: Default::default(),
            near: Default::default(), 
            fov_w: Default::default(), 
            view_w: Default::default(), 
            view_h: Default::default(), 
            vw_step: Default::default(), 
            vh_step: Default::default(), 
            sample_pre_pixel : 1,
        }
    }
}

impl Camera {
    pub fn new(w:u32, h:u32, pos:Vector3, dir:Vector3, dir_w:Vector3, near:f64, fov_w:f64,sample_pre_pixel : u64) -> Camera{
        let view_w = (fov_w / 2f64).to_radians().tan() * near;
        let view_h = view_w * h as f64 / w as f64;
        let vw_step = view_w / w as f64;
        let vh_step = view_h / h as f64;
        let dir_up = dir_w.cross(&dir);
        Camera{
            screen_h:h,
            screen_w:w,
            pos:pos,
            dir:dir.normallized(),
            dir_w:dir_w.normallized(),
            dir_up,

            near,
            fov_w,
            view_w,
            view_h,

            vw_step,
            vh_step,
            sample_pre_pixel,
        }
    }

    pub fn get_w(&self) -> u32{
        self.screen_w
    }

    pub fn get_h(&self) -> u32{
        self.screen_h
    }

    pub fn get_view_w(&self) -> f64{
        self.view_w
    }

    pub fn get_view_h(&self) -> f64{
        self.view_h
    }

    pub fn get_pos(&self) -> Vector3{
        self.pos
    }

    pub fn get_piexl_view_pos_start_pos(&self) -> Vector3{
        let veiew_center = self.pos + self.dir * self.near;
        let view_start_pos = veiew_center 
            - self.dir_up * (self.view_h * (0.5f64 + self.vh_step * 0.5)) 
            - self.dir_w * (self.view_w * (0.5f64 + self.vw_step * 0.5));
        return view_start_pos;
    }

    ///render scene with camera
    pub fn render(&self, s:&Scene, depth:u64) -> Img{
        let sw = self.get_w();
        let sh = self.get_h();

        let mut img = rust_tiny_img::img::Img::new(sw, sh);
        let start_pos = self.get_piexl_view_pos_start_pos();
        let mut rng = thread_rng();

        println!("start render");

        for h in 0..sh {
            println!("remaining:{} row",sh - h);
            for w in 0..sw {
                // let sample_pre_pixel = self.sample_pre_pixel;
                // let handle = thread::spawn(move || {
                //     let color : Color;
                //     let start_pos = self.get_piexl_view_pos_start_pos();
                //     let mut rng = thread_rng();
                //     if sample_pre_pixel > 1 {
                //         let mut r = 0.0;
                //         let mut g = 0.0;
                //         let mut b = 0.0;
                //         let rate: f64 = 1.0 / sample_pre_pixel as f64;
                //         for i in 0..sample_pre_pixel {
                //             let ray = self.get_ray(&start_pos,w,h,&mut rng);
                //             let color = ray.get_color(s,depth);
                //             r += color.r() as f64;
                //             g += color.g() as f64;
                //             b += color.b() as f64;
                //         }
                //     }
                // });
                let color;
                if self.sample_pre_pixel > 1 {
                    // let mut final_color = Color::get_black();
                    let mut r = 0.0;
                    let mut g = 0.0;
                    let mut b = 0.0;
                    let rate: f64 = 1.0 / self.sample_pre_pixel as f64;
                    for i in 0..self.sample_pre_pixel {
                        let ray = self.get_ray(&start_pos,w,h,&mut rng);
                        let color = ray.get_color(s,depth);
                        r += color.r() as f64;
                        g += color.g() as f64;
                        b += color.b() as f64;
                    }
                    let cr = (r * rate) as u8;
                    let cg = (g * rate) as u8;
                    let cb = (b * rate) as u8;
                    color = Color::new(cr, cg, cb, 255u8);
                }
                else {
                    let dir = self.get_piexl_view_pos(&start_pos, w, h);
                    let ray = Ray::new(self.get_pos(),dir - self.get_pos());
                    color = ray.get_color(s,depth);
                }
                match img.set_pixel(w, h, color) {
                    Err(msg) => {
                        panic!("{}",msg);
                    },
                    _ => (),
                }
            }
        }
        img
    }

    ///获取像素点对应的位置 可以用来生成Ray
    fn get_piexl_view_pos(&self, view_start_pos:&Vector3, x:u32, y:u32) -> Vector3{
        view_start_pos.clone() 
            + self.dir_up * self.vh_step * y as f64 
            + self.dir_w * self.vw_step * x as f64
    }

    fn get_ray(&self, start_pos:&Vector3, x:u32, y:u32, rng:&mut ThreadRng) -> Ray {
        let random_x = rng.gen_range(-0.5f64..0.5f64);
        let random_y = rng.gen_range(-0.5f64..0.5f64);

        let dir = start_pos.clone() 
            + self.dir_up * self.vh_step * (y as f64 + random_y)  
            + self.dir_w * self.vw_step * (x as f64 + random_x) ;
        let ray = Ray::new(self.get_pos(),dir - self.get_pos());
        return ray;
    }
}
