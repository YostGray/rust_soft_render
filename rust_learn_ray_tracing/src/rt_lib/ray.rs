use rust_tiny_img::color::Color;

use super::scene::Scene;
use super::{vector3::Vector3, geometry::Geometry};
use super::geometry::HitReultEnum;

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

    pub fn get_color(&self,s : &Scene) -> Color{
        match s.try_hit(self) {
            HitReultEnum::Ruslt(r) => {
                let n = r.get_normal();
                Color::new((n.get_x() * 255.0) as u8, (n.get_y() * 255.0) as u8, (n.get_z() * 255.0) as u8, 255u8)
            },
            HitReultEnum::None => {
                let test = 0.5 * (self.dir.get_y() + 1.0);
                Color::get_white() *(1.0 - test) + Color::new(127u8, 178u8, 255u8, 255u8) * test
            },
        }
    }
}