use super::{ray::Ray, vector3::Vector3};

pub struct HitResult{
    pos : Vector3,
    normal : Vector3,
    t : f64,
    reflect_dir : Vector3,
}

impl HitResult {
    pub fn new(pos: Vector3, normal: Vector3, t: f64, reflect_dir: Vector3) -> Self { Self { pos, normal, t , reflect_dir} }
    pub fn get_pos(&self) -> &Vector3 {&self.pos}
    pub fn get_normal(&self) -> &Vector3 {&self.normal}
    pub fn get_t(&self) -> f64 {self.t}
    pub fn get_reflect_dir(&self) -> &Vector3 {&self.reflect_dir}
}

pub enum HitReultEnum{
    None,
    Ruslt(HitResult),
}

pub trait Geometry {
    fn try_hit(&self,ray:&Ray) -> HitReultEnum;
}