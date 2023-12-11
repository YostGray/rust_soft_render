use std::f64::INFINITY;

use super::geometry::{Geometry, HitReultEnum};


///the scene which would be render
pub struct Scene {
    obj_list:Vec<Box<dyn Geometry + Sync + Send>>,
}

impl Scene {
    pub fn new() -> Self { 
        Self { 
            obj_list : vec![],
        } 
    }

    pub fn add_obj(&mut self,obj:Box<dyn Geometry + Sync + Send>) {
        self.obj_list.push(obj);
    }

    fn get_obj_vec(&self) -> &Vec<Box<dyn Geometry + Sync + Send>>{
        &self.obj_list
    }
}

impl Geometry for Scene {
    fn try_hit(&self,ray:&super::ray::Ray) -> HitReultEnum {
        let mut min_t = INFINITY;
        let mut result = HitReultEnum::None;
        for o in &self.obj_list {
            match o.try_hit(ray) {
                HitReultEnum::Ruslt(r) => {
                    if min_t > r.get_t() {
                        min_t = r.get_t();
                        result = HitReultEnum::Ruslt(r);
                    }
                },
                HitReultEnum::None => (),
            }
        }
        result
    }
}