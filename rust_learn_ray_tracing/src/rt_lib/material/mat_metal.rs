use rand::thread_rng;

use crate::rt_lib::{ray::Ray, vector3::Vector3, geometry::HitResult};

pub struct Metal {
    albedo : Vector3,
    fuzz : f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Self { Self { albedo , fuzz} }
}

impl super::Mat for Metal {
    fn gen_mat_result(&self, hit_result:&mut HitResult){
        let n = hit_result.get_normal();
        let i = -hit_result.get_in_dir().clone();
        let mut o = -i + n * 2.0 * i.dot(n);
        if self.fuzz > 0.0001 {
            let mut rng = thread_rng();
            let mut random = Vector3::random(&mut rng, -1.0, 1.0);
            random.normallize();
            while random.dot(&o) < 0.0001 {
                random = Vector3::random(&mut rng, -1.0, 1.0);
                random.normallize();
            }
            random *= self.fuzz;
            o += random;
        }

        hit_result.set_out_dir(o);
        hit_result.set_eval_color(self.albedo);
    }
}