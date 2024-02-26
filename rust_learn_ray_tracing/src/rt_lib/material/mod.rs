use super::{ray::Ray, vector3::Vector3, geometry::HitResult};

pub mod mat_lambertian;
pub mod mat_metal;
pub mod mat_dielectric;

/// 材质需要哪些数据作为输入，哪些作为输出？
/// 输入：
///     命中点信息(入射方向 法线 击中位置) *光线之前所在物体折射率(暂时不考虑互相包含的情况)
/// 输出：
///     反射\折射方向 光线反弹\吸收信息 
pub trait Mat {
    // fn gen_out_dir(&self, into:&Vector3, normal:&Vector3) -> Vector3;
    // fn get_mult_value(&self, into:&Vector3, normal:&Vector3) -> Vector3;
    fn gen_mat_result(&self, hit_result:&mut HitResult){}
}