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
    fn gen_out_dir(&self, into:&Vector3, normal:&Vector3) -> Vector3 {
        let i = -into.clone();
        let mut o = -i + normal * 2.0 * i.dot(normal);

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

        return o;
    }

    fn get_mult_value(&self, into:&Vector3, normal:&Vector3) -> Vector3 {
        self.albedo
    }
}