use super::vector3::Vector3;

/// the view point to render scene.
/// use left hand coordinary.
pub struct Camera {
    screen_w : u32,
    screen_h : u32,

    pos : Vector3,
    dir : Vector3,
    dir_w : Vector3,
    dir_up : Vector3,

    near : f64,
    fov_w  : f64,//degree
    view_w : f64,
    view_h : f64,
    
    vw_step : f64,
    vh_step : f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
            screen_w: Default::default(), 
            screen_h: Default::default(), 
            pos: Default::default(), 
            dir: Default::default(), 
            dir_w: Default::default(), 
            dir_up: Default::default(),
            near: Default::default(), 
            fov_w: Default::default(), 
            view_w: Default::default(), 
            view_h: Default::default(), 
            vw_step: Default::default(), 
            vh_step: Default::default(), 
        }
    }
}

impl Camera {
    pub fn new(w:u32, h:u32, pos:Vector3, dir:Vector3, dir_w:Vector3, near:f64, fov_w:f64) -> Camera{
        let view_w = (fov_w / 2f64).to_radians().tan() * near;
        let view_h = view_w * h as f64 / w as f64;
        let vw_step = view_w / w as f64;
        let vh_step = view_h / h as f64;
        let dir_up = dir_w.cross(&dir);
        Camera{
            screen_h:h,
            screen_w:w,
            pos:pos,
            dir:dir.normallized(),
            dir_w:dir_w.normallized(),
            dir_up,

            near,
            fov_w,
            view_w,
            view_h,

            vw_step,
            vh_step,
        }
    }

    pub fn get_w(&self) -> u32{
        self.screen_w
    }

    pub fn get_h(&self) -> u32{
        self.screen_h
    }

    pub fn get_view_w(&self) -> f64{
        self.view_w
    }

    pub fn get_view_h(&self) -> f64{
        self.view_h
    }

    pub fn get_pos(&self) -> Vector3{
        self.pos
    }

    ///获取像素点对应的位置 可以用来生成Ray
    pub fn get_piexl_view_pos(&self, view_start_pos:Vector3, x:u32, y:u32) -> Vector3{
        view_start_pos + self.dir_up * self.vh_step * y as f64 + self.dir_w * self.vw_step * x as f64
    }

    pub fn get_piexl_view_pos_start_pos(&self) -> Vector3{
        let veiew_center = self.pos + self.dir * self.near;
        let view_start_pos = veiew_center - self.dir_up * (self.view_h * 0.5f64) - self.dir_w * (self.view_w * 0.5f64);
        return view_start_pos;
    }
}
