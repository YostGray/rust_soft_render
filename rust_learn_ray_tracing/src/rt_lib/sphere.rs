use std::{rc::Rc, sync::Arc};

use rand::thread_rng;

use super::{vector3::Vector3, geometry::{Geometry, HitResult}, ray::Ray, material::Mat};

pub struct Sphere{
    r:f64,
    pos:Vector3,
    mat:Arc<dyn Mat + Send + Sync>,
}

impl Sphere {
    pub fn new(pos:&Vector3,r:f64,mat:Arc<dyn Mat + Send + Sync>) -> Sphere {
        Sphere{
            r,
            pos:pos.clone(),
            mat,
        }
    }
}

impl Geometry for Sphere {
    fn try_hit(&self,ray:&Ray) -> Option<HitResult> {
        let cq = self.pos - ray.get_ori().clone();
        let a = ray.get_dir().dot(ray.get_dir());
        let b = (ray.get_dir() * -2.0f64).dot(&cq);
        let c = cq.dot(&cq) - self.r * self.r;
        let discriminant  = b * b - 4.0 * a * c;
        match discriminant < 0.0 {
            true => Option::None,
            false => {
                let sqrtd = discriminant.sqrt();
                let mut t = (-b - sqrtd)/(a * 2.0);
                if t < 0.0 {
                    t = (-b + sqrtd)/(a * 2.0);
                }
                if t < 1e-10 {
                    return Option::None;
                }
                let pos = ray.at(t);
                let mut normal = pos - self.pos;
                normal.normallize();
                let mat = Arc::clone(&self.mat);
                Option::Some(HitResult::new(ray, t, pos, normal, &mat))
            },
        }
    }
}