use super::scene::Scene;
use super::{vector3::Vector3, geometry::Geometry};

pub struct Ray{
    ori : Vector3,
    dir : Vector3,
}

impl Ray {
    pub fn new(ori: Vector3, dir: Vector3) -> Self { 
        Self { ori, dir:dir.normallized() } 
    }

    pub fn get_ori(&self) -> &Vector3 {
        &self.ori
    }

    pub fn get_dir(&self) -> &Vector3 {
        &self.dir
    }
    
    pub fn at(&self, t:f64) -> Vector3 {
        self.ori + self.dir * t
    }

    pub fn get_color(&self, s:&Scene, depth:u64) -> Vector3{
        if depth <= 0  {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        match s.try_hit(self) {
            Option::Some(r) => {
                let reflect_ray = Ray::new(r.get_pos().clone(), r.get_reflect_dir().clone());
                let m = r.get_mat();
                let reflect_rate = m.get_mult_value(r.get_into_dir(), r.get_normal());
                reflect_rate * (reflect_ray.get_color(s,depth - 1) * 0.5)
            },
            Option::None => {
                let test = 0.5 * (self.dir.get_y() + 1.0);
                Vector3::new(1.0, 1.0, 1.0) * (1.0 - test) + Vector3::new(127.0 / 255.0, 178.0 / 255.0, 255.0 / 255.0) * test
            },
        }
    }
}