use super::{ray::Ray, vector3::Vector3, geometry::HitResult};

pub mod mat_lambertian;
pub mod mat_metal;

pub trait Mat {
    fn gen_out_dir(&self, into:&Vector3, normal:&Vector3) -> Vector3;
    fn get_mult_value(&self, into:&Vector3, normal:&Vector3) -> Vector3;
}