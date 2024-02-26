use std::f64::INFINITY;

use super::geometry::{Hitable, HitResult};


///the scene which would be render
pub struct Scene {
    obj_list:Vec<Box<dyn Hitable + Sync + Send>>,
}

impl Scene {
    pub fn new() -> Self { 
        Self { 
            obj_list : vec![],
        } 
    }

    pub fn add_obj(&mut self,obj:Box<dyn Hitable + Sync + Send>) {
        self.obj_list.push(obj);
    }

    fn get_obj_vec(&self) -> &Vec<Box<dyn Hitable + Sync + Send>>{
        &self.obj_list
    }
}

impl Hitable for Scene {
    fn try_hit(&self,ray:&super::ray::Ray) -> Option<HitResult> {
        let mut min_t = INFINITY;
        let mut result = Option::None;
        for o in &self.obj_list {
            match o.try_hit(ray) {
                Option::Some(r) => {
                    if min_t > r.get_t() {
                        min_t = r.get_t();
                        result = Option::Some(r);
                    }
                },
                Option::None => (),
            }
        }
        result
    }
}