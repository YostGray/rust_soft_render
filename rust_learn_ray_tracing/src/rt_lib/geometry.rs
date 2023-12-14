use std::sync::Arc;

use super::{ray::{Ray, self}, vector3::Vector3,material::Mat};

pub struct HitResult {
    pos : Vector3,
    normal : Vector3,
    t : f64,
    is_front_face :bool,

    in_dir : Vector3,
    out_dir : Vector3,

    mat : Box<Arc<dyn Mat + Send + Sync>>,
    eval_color : Vector3,
}

impl HitResult {
    pub fn new(ray_i:&Ray, t:f64, pos:Vector3, normal:Vector3, m:&Arc<dyn Mat + Send + Sync>) -> Self {
        let mat = Box::new(Arc::clone(m));
        let mut hr = Self { 
            pos, 
            normal, 
            t, 
            is_front_face:true,
            out_dir: Vector3::default(), 
            mat, 
            in_dir:ray_i.get_dir().clone(),
            eval_color:Vector3::default(),
        };
        hr.judge_is_front_face();
        let mat = hr.get_mat();
        mat.gen_mat_result(&mut hr);
        return hr;
    }
    fn judge_is_front_face(&mut self) {
        self.is_front_face = self.in_dir.dot(&self.normal) < 0.0;
        if !self.is_front_face {
            self.normal = -self.normal;
        }
    }

    pub fn set_out_dir(&mut self,out_dir : Vector3){self.out_dir = out_dir}
    pub fn set_eval_color(&mut self,eval_color : Vector3){self.eval_color = eval_color}

    pub fn get_pos(&self) -> &Vector3 {&self.pos}
    pub fn get_normal(&self) -> &Vector3 {&self.normal}
    pub fn get_t(&self) -> f64 {self.t}
    pub fn get_is_front_face(&self) -> bool {self.is_front_face}
    pub fn get_out_dir(&self) -> &Vector3 {&self.out_dir}
    pub fn get_in_dir(&self) -> &Vector3 {&self.in_dir}
    pub fn get_mat(&self) -> Arc<dyn Mat + Send + Sync> { Arc::clone(&self.mat) }
    pub fn get_eval_color(&self) -> &Vector3 { &self.eval_color }
}

pub enum HitReultEnum{
    None,
    Ruslt(HitResult),
}

pub trait Geometry {
    fn try_hit(&self,ray:&Ray) -> Option<HitResult>;
}