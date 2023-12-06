use super::vector3::Vector3;

struct Ray{
    ori : Vector3,
    dir : Vector3,
}

impl Ray {
    pub fn new(ori: Vector3, dir: Vector3) -> Self { 
        Self { ori, dir } 
    }

    pub fn get_ori(&self) -> &Vector3 {
        &self.ori
    }
    
    pub fn get_dir(&self) -> &Vector3 {
        &self.dir
    }
    
    pub fn at(&self, t:f64) -> Vector3 {
        &self.ori + &(&self.dir * t)
    }
}