use rand::thread_rng;

use crate::rt_lib::{ray::Ray, vector3::Vector3, geometry::HitResult};

pub struct Lambertian {
    albedo : Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self { Self { albedo } }
}

impl super::Mat for Lambertian {
    fn gen_mat_result(&self, hit_result:&mut HitResult){
        let n = hit_result.get_normal();
        let mut rng = thread_rng();
        let mut o = Vector3::random_unit_vector(&mut rng) + *n;
        o = match o.dot(n) > 0.0 {
            true => o,
            false => -o,
        };
        o.normallize();

        hit_result.set_out_dir(o);
        hit_result.set_eval_color(self.albedo);
    }
}