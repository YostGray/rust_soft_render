use super::color::Color;


/// Abstract img with width, height and pixels.
/// 
/// When read a file, it will be converted to this. 
/// 
/// Then we can modify it and save as another name or format.
#[derive(Clone)]
pub struct Img {
    width : u32,
    height : u32,
    pixels : Vec<Color>, //line first
}

impl Img {
    pub fn new(width : u32,height : u32) -> Img {
        let black = Color::new(255,255,255, 255);
        Img {
            width,
            height,
            pixels:vec![black;(width * height) as usize],
        }
    }

    pub fn get_pixels(&self) -> &Vec<Color> {
        return &self.pixels;
    }

    pub fn get_pixel(&self, x:u32, y:u32) -> Option<&Color> {
        let index = self.get_index(x,y);
        self.pixels.get(index)
    }

    pub fn set_pixel(&mut self, x:u32, y:u32, color:Color) -> Result<(),&'static str> {
        if x >= self.width || y >= self.height{
            return Err("set pixel err, pos out of image size");
        }
        let index = self.get_index(x,y);
        self.pixels[index] = color;
        Ok(())
    }

    #[inline]
    pub fn get_index(&self, x:u32, y:u32) -> usize {
        (self.width * y + x) as usize
    }

    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    pub fn get_bytes_size(&self) -> u32 {
        4 * self.pixels.len() as u32
    }
}