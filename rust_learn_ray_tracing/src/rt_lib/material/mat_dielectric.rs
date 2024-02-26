use rand::{thread_rng, Rng};

use crate::rt_lib::{ray::Ray, vector3::Vector3, geometry::HitResult};

pub struct Dielectric  {
    inside_rate : f64,
}

impl Dielectric {
    pub fn new(inside_rate: f64) -> Self { Self {inside_rate} }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // 使用 Schlick 的反射近似。
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0 ) * (1.0 - cosine).powf(5.0)
    }
}

impl super::Mat for Dielectric {
    fn gen_mat_result(&self, hit_result:&mut HitResult){
        let i = hit_result.get_in_dir().clone(); 
        let mut n = hit_result.get_normal().clone(); 

        let out_rate: f64 = 1.0;// always air
        let refraction_ratio = match hit_result.get_is_front_face() {
            true => {
                out_rate / self.inside_rate
            },
            false => {
                self.inside_rate / out_rate
            },
        };

        let cos_theta = (-i).dot(&n).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let mut rng = thread_rng();
        if refraction_ratio * sin_theta > 1.0 || Dielectric::reflectance(cos_theta,refraction_ratio) > rng.gen_range(0.0..1.0) {
            let n = hit_result.get_normal();
            let i = -hit_result.get_in_dir().clone();
            let mut o = -i + n * 2.0 * i.dot(n);
            hit_result.set_out_dir(o);
            hit_result.set_eval_color(Vector3::new(1.0, 1.0, 1.0));
        } else 
        {
            let r_out_perp =  (i + n * cos_theta) * refraction_ratio;
            let r_out_parallel = n * -(1.0 - r_out_perp.length_sqr()).abs().sqrt();
            hit_result.set_out_dir(r_out_perp + r_out_parallel);
            hit_result.set_eval_color(Vector3::new(1.0, 1.0, 1.0));
        }
    }
}