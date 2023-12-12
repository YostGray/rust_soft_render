use rand::thread_rng;

use super::{vector3::Vector3, geometry::{Geometry, HitReultEnum, HitResult}, ray::Ray};

pub struct Sphere{
    r:f64,
    pos:Vector3,
}

impl Sphere {
    pub fn new(pos:&Vector3,r:f64) -> Sphere {
        Sphere{
            r,
            pos:pos.clone(),
        }
    }

    fn get_reflect_dir(&self, ray:&Ray, normal:&Vector3) -> Vector3 {
        // //镜面反射
        // let n = normal;
        // let i = -ray.get_dir().clone();
        // let o = -i + n * 2.0 * i.dot(n);
        // return o;

        //随机漫反射
        let mut rng = thread_rng();
        let mut o = Vector3::random(&mut rng, -1.0, 1.0) + *normal;
        o.normallize();
        match o.dot(normal) > 0.0 {
            true => o,
            false => -o,
        }
    }
}

impl Geometry for Sphere {
    fn try_hit(&self,ray:&Ray) -> HitReultEnum {
        let cq = self.pos - ray.get_ori().clone();
        let a = ray.get_dir().dot(ray.get_dir());
        let b = (ray.get_dir() * -2.0f64).dot(&cq);
        let c = cq.dot(&cq) - self.r * self.r;
        let discriminant  = b * b - 4.0 * a * c;
        match discriminant < 0.0 {
            true => HitReultEnum::None,
            false => {
                let sqrtd = discriminant.sqrt();
                let mut t = (-b - sqrtd)/(a * 2.0);
                if t < 0.0 {
                    t = (-b + sqrtd)/(a * 2.0);
                }
                if t < 1e-10 {
                    return HitReultEnum::None;
                }
                let pos = ray.at(t);
                let mut normal = pos - self.pos;
                normal.normallize();
                let reflect_dir = self.get_reflect_dir(ray,&normal);
                HitReultEnum::Ruslt(HitResult::new(pos, normal, t, reflect_dir))
            },
        }
    }
}