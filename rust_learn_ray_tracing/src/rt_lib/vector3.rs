use std::{ops::{Add, Sub, Neg, AddAssign, Mul, Div, DivAssign, MulAssign}, f64::consts::PI};

use rand::{rngs::ThreadRng, Rng};


///Vector, which could present a pint or a vector quantity.
#[derive(Debug, Clone, Copy)]
pub struct Vector3{
    x : f64,
    y : f64,
    z : f64,
}

impl Vector3 {
    pub fn new(x: f64,y: f64,z: f64) -> Vector3 {
        Vector3{x,y,z}
    }

    pub fn get_x(&self) -> f64 {
        return self.x;
    }
    pub fn get_y(&self) -> f64 {
        return self.y;
    }
    pub fn get_z(&self) -> f64 {
        return self.z;
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new(0f64, 0f64, 0f64)
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (*self - *other).length_sqr() < 1e-8f64
    }
}

//math operations 👇

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x:self.x + rhs.x,
            y:self.y + rhs.y,
            z:self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x:self.x - rhs.x,
            y:self.y - rhs.y,
            z:self.z - rhs.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x:-self.x,
            y:-self.y,
            z:-self.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x:self.x * rhs.x,
            y:self.y * rhs.y,
            z:self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x:self.x * rhs,
            y:self.y * rhs,
            z:self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x:self.x * rhs,
            y:self.y * rhs,
            z:self.z * rhs,
        }
    }
}


impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        //mult is faster
        let temp = 1f64 / rhs;
        Vector3 {
            x:self.x * temp,
            y:self.y * temp,
            z:self.z * temp,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        //mult is faster
        let temp = 1f64 / rhs;
        self.x *= temp;
        self.y *= temp;
        self.z *= temp;
    }
}

impl Vector3 {
    pub fn dot(&self, other:&Vector3) -> f64{
            self.x * other.x 
                + self.y * other.y 
                + self.z * other.z
    }

    pub fn cross(&self, other:&Vector3) -> Vector3{
        Self {
            x:self.y * other.z - self.z * other.y,
            y:self.z * other.x - self.x * other.z,
            z:self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_sqr(&self) -> f64{
        self.x * self.x 
            + self.y * self.y 
            + self.z * self.z
    }

    pub fn length(&self) -> f64{
        self.length_sqr().sqrt()
    }

    pub fn normallize(&mut self){
        *self /= self.length()
    }

    pub fn normallized(&self) -> Vector3{
        *self / self.length()
    }

    pub fn random(rng:&mut ThreadRng,min:f64,max:f64) -> Vector3 {
        Vector3 {
            x:rng.gen_range(min..max),
            y:rng.gen_range(min..max),
            z:rng.gen_range(min..max),
        }
    }
    pub fn random_unit_vector(rng:&mut ThreadRng) -> Vector3 {
        let a = rng.gen_range(0.0..2.0*PI);
        let z = rng.gen_range(-1.0.. 1.0);
        let r = (1.0f64 - z*z).sqrt();
        Vector3 {
            x:r * a.cos(),
            y:r * a.sin(),
            z,
        }
    }
}

#[cfg(test)]
mod test_vector3 {
    use super::*;

    #[test]
    fn sample_tests() {
        let one = Vector3::new(1f64, 1f64, 1f64);
        let half_one = Vector3::new(0.5f64, 0.5f64, 0.5f64);
        let mut one_mut = Vector3::new(1f64, 1f64, 1f64);

        assert_eq!(one,one_mut);

        one_mut.normallize();
        assert_eq!(one.normallized(),one_mut);

        assert_ne!(one,one_mut);
        assert_eq!(one * 2f64,one + one);
        assert_eq!(one / 2f64,half_one);
        assert_eq!(one - one,Vector3::default());

        assert_eq!(one.dot(&one),3f64);

        let toward_x = Vector3::new(1f64, 0f64, 0f64);
        let toward_y = Vector3::new(0f64, 1f64, 0f64);
        let toward_z = Vector3::new(0f64, 0f64, 1f64);
        assert_eq!(toward_x.cross(&toward_y),toward_z);
    }
}