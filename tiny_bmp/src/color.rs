#[derive(Debug)]
pub struct Color
{
    r : u8,
    g : u8,
    b : u8,
}

impl Color 
{
    pub fn new (r: u8,g: u8,b: u8) -> Color 
    {
        Color{ r, g, b, }
    }

    pub fn r (&self) -> u8 
    {
        return self.r;
    }

    pub fn g (&self) -> u8
     {
        return self.g;
    }

    pub fn b (&self) -> u8 
    {
        return self.b;
    }

    pub fn color_to_gray(&mut self) {
        let rg = self.r as f64 * 0.2126;
        let gg = self.g as f64 * 0.7152;
        let bg = self.b as f64 * 0.0722;
        let gray = (rg + gg + bg).round() as u8;
        self.r = gray;
        self.g = gray;
        self.b = gray;
    }

    // #[inline]
    // pub fn color_to_gray_AVX(&mut self,m: std::arch::x86_64::__m256d)
    // {
    //     if is_x86_feature_detected!("avx") 
    //     {
    //         unsafe 
    //         {
    //             let s = std::arch::x86_64::_mm256_set_pd(self.r as f64, self.g as f64, self.b as f64, 0.0);
    //             // let m = std::arch::x86_64::_mm256_set_pd(0.2126, 0.7152, 0.0722, 0.0);
    //             let sm = std::arch::x86_64::_mm256_mul_pd(s, m);

    //             let unpacked: (f64, f64, f64, f64) = std::mem::transmute(sm);

    //             let gray = (unpacked.1 + unpacked.2 + unpacked.3).round() as u8;
    //             self.r = gray;
    //             self.g = gray;
    //             self.b = gray;
    //         }
    //     }
    //     else {
    //         self.color_to_gray()
    //     }
    // }

}

#[cfg(test)]
mod test_color {
    use super::*;

    #[test]
    fn sample_tests() {
        let mut red = Color::new(255, 255, 255);
        red.color_to_gray();
        println!("{:?}",red);
    }
}