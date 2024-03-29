use std::{thread, sync::{Arc, Mutex}, time::Duration, io::{BufWriter, self}};

use rust_tiny_img::{img::Img, color::{Color, self}};
use rand::{Rng, thread_rng, rngs::ThreadRng};
use crate::rt_lib::{ray::Ray, vector3};
use super::{vector3::Vector3, scene::Scene};

/// the view point to render scene.
/// use left hand coordinary.
#[derive(Clone, Copy)]
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
    ray_deepth:u64,
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
            ray_deepth : 50,
        }
    }
}

impl Camera {
    pub fn new(w:u32, h:u32, pos:Vector3, dir:Vector3, dir_w:Vector3, near:f64, fov_w:f64,sample_pre_pixel : u64,ray_deepth : u64) -> Camera{
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
            ray_deepth,
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
    pub fn render(&self, s:Scene, depth:u64) -> Img{
        let sw = self.get_w();
        let sh = self.get_h();

        let mut img = rust_tiny_img::img::Img::new(sw, sh);

        println!("start render");

        let start_pos = self.get_piexl_view_pos_start_pos();

        let shared_s = Arc::new(s);
        let shared_c = Arc::new(self.clone());
        let shared_img = Arc::new(Mutex::new(img));

        let mut thread_num = 0;
        let shared_thread_num = Arc::new(Mutex::new(thread_num));

        let mut std_out: BufWriter<io::Stdout> = BufWriter::new(io::stdout());
        for h in 0..sh {
            let depth_copy = depth;
            let shared_s_clone = Arc::clone(&shared_s);
            let shared_c_clone = Arc::clone(&shared_c);
            let shared_img_clone = Arc::clone(&shared_img);
            let shared_thread_num_clone = Arc::clone(&shared_thread_num);

            let handle = thread::spawn(move || {
                let mut rng = thread_rng();
                {
                    let mut num = shared_thread_num_clone.lock().unwrap();
                    *num += 1;
                }
                for w in 0..sw {
                    let start_pos = &shared_c_clone.get_piexl_view_pos_start_pos();
                    let color = match shared_c_clone.sample_pre_pixel > 1 {
                        true => {
                            let mut out_vector3 = Vector3::new(0.0, 0.0, 0.0);
                            let rate: f64 = 255.0 / shared_c_clone.sample_pre_pixel as f64;
                            for i in 0..shared_c_clone.sample_pre_pixel {
                                let ray = Camera::get_ray(&shared_c_clone,start_pos,w,h,&mut rng);
                                let vector3 = ray.get_color(&shared_s_clone,depth);
                                out_vector3 += vector3;
                            }
                            out_vector3 *= rate;
                            let cr = out_vector3.get_x() as u8;
                            let cg = out_vector3.get_y() as u8;
                            let cb = out_vector3.get_z() as u8;
                            Color::new(cr, cg, cb, 255u8)
                        },
                        false => {
                            let dir = shared_c_clone.get_piexl_view_pos(start_pos,w,h);
                            let pos = shared_c_clone.pos;
                            let ray = Ray::new(pos,dir - pos);
                            let out_vector3 = ray.get_color(&shared_s_clone,depth_copy) * 255.0;
                            let cr = out_vector3.get_x() as u8;
                            let cg = out_vector3.get_y() as u8;
                            let cb = out_vector3.get_z() as u8;
                            Color::new(cr, cg, cb, 255u8)
                        }
                    };
                    match shared_img_clone.lock().unwrap().set_pixel(w, h, color) {
                        Err(msg) => {
                            panic!("{}",msg);
                        },
                        _ => (),
                    }
                }
                {
                    let mut num = shared_thread_num_clone.lock().unwrap();
                    *num -= 1;
                }
            }); 

            //大于16个线程时
            while *shared_thread_num.lock().unwrap() > 16 {

            }
            crate::rt_lib::show_progress(&mut std_out, (h as f64  * 100.0/ sh as f64 ).ceil());
        }
        while *shared_thread_num.lock().unwrap() > 0 {

        }
        let img = (*shared_img.lock().unwrap()).clone();
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
