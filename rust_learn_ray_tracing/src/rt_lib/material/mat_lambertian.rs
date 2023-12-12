use rand::thread_rng;

use crate::rt_lib::{ray::Ray, vector3::Vector3, geometry::HitResult};

pub struct Lambertian {
    albedo : Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self { Self { albedo } }
}

impl super::Mat for Lambertian {
    fn gen_out_dir(&self, into:&Vector3, normal:&Vector3) -> Vector3 {
        let mut rng = thread_rng();
        let mut o = Vector3::random(&mut rng, -1.0, 1.0) + *normal;
        o.normallize();
        match o.dot(normal) > 0.0 {
            true => o,
            false => -o,
        }
    }

    fn get_mult_value(&self, into:&Vector3, normal:&Vector3) -> Vector3 {
        self.albedo
    }
}