
///Color, which is basic unit of img.
#[derive(Debug,Clone)]
pub struct Color{
    r : u8,
    g : u8,
    b : u8,
    a : u8,
}

impl Color {
    pub fn new (r: u8,g: u8,b: u8,a: u8) -> Color {
        Color{ r, g, b, a}
    }

    pub fn r (&self) -> u8 {
        return self.r;
    }

    pub fn g (&self) -> u8{
        return self.g;
    }

    pub fn b (&self) -> u8 {
        return self.b;
    }

    pub fn a (&self) -> u8 {
        return self.a;
    }

    ///Convert color to gray scale
    pub fn color_to_gray(&mut self) {
        let rg = self.r as f64 * 0.2126;
        let gg = self.g as f64 * 0.7152;
        let bg = self.b as f64 * 0.0722;
        let gray = (rg + gg + bg).round() as u8;
        self.r = gray;
        self.g = gray;
        self.b = gray;
    }

    ///linear interpolation two colors
    pub fn linear_interpolation(color_a : &Color, color_b : &Color, a_rate : f64) -> Result<Color,&'static str> {
        if a_rate > 1.0 || a_rate < 0.0 {
            return Err("color linear interpolation err: rate rang wrong");
        }
        let b_rate = 1.0 - a_rate;
        let r = (color_a.r as f64 * a_rate + color_b.r as f64 * b_rate) as u8;
        let g = (color_a.g as f64 * a_rate + color_b.g as f64 * b_rate) as u8;
        let b= (color_a.b as f64 * a_rate + color_b.b as f64 * b_rate) as u8;
        let a= (color_a.a as f64 * a_rate + color_b.a as f64 * b_rate) as u8;
        return Ok(Color::new(r, g, b, a));
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

#[cfg(test)]
mod test_color {
    use super::*;

    #[test]
    fn sample_tests() {
        let mut black = Color::new(255, 255, 255, 255);
        let mut white = Color::new(0, 0, 0, 255);
        black.color_to_gray();
        white.color_to_gray();
        assert_eq!(black,Color::new(255, 255, 255, 255));
        assert_eq!(white,Color::new(0, 0, 0, 255));

        let mid_color_result = Color::linear_interpolation(&black,&white,0.5);
        assert!(mid_color_result.is_ok());
        let mid_color = mid_color_result.unwrap();
        assert!(mid_color.r >= 127 && mid_color.r <= 128);//float precision
    }
}