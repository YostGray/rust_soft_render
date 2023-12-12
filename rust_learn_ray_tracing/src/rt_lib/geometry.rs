use std::sync::Arc;

use super::{ray::{Ray, self}, vector3::Vector3,material::Mat};

pub struct HitResult {
    pos : Vector3,
    normal : Vector3,
    t : f64,
    reflect_dir : Vector3,
    into_dir : Vector3,
    mat : Box<Arc<dyn Mat + Send + Sync>>,
}

impl HitResult {
    pub fn new(ray_i:&Ray, t:f64, pos:Vector3, normal:Vector3, m:&Arc<dyn Mat + Send + Sync>) -> Self {
        let mut reflect_dir= m.gen_out_dir(ray_i.get_dir(), &normal);
        let mat = Box::new(Arc::clone(m));
        Self { pos, normal, t , reflect_dir , mat , into_dir:ray_i.get_dir().clone()}
    }
    pub fn get_pos(&self) -> &Vector3 {&self.pos}
    pub fn get_normal(&self) -> &Vector3 {&self.normal}
    pub fn get_t(&self) -> f64 {self.t}
    pub fn get_reflect_dir(&self) -> &Vector3 {&self.reflect_dir}
    pub fn get_into_dir(&self) -> &Vector3 {&self.into_dir}
    pub fn get_mat(&self) -> Arc<dyn Mat + Send + Sync> { Arc::clone(&self.mat) }
}

pub enum HitReultEnum{
    None,
    Ruslt(HitResult),
}

pub trait Geometry {
    fn try_hit(&self,ray:&Ray) -> Option<HitResult>;
}