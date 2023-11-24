use super::color::Color;

pub struct Img {
    width : u32,
    height : u32,
    pixels : Vec<Color>, //line first
}

impl Img {
    pub fn new(width : u32,height : u32) -> Img {
        let black = Color::new(255,255,255);
        Img {
            width,
            height,
            pixels:vec![black;(width * height) as usize],
        }
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

    pub(crate) fn get_width(&self) -> u32 {
        return self.width;
    }

    pub(crate) fn get_height(&self) -> u32 {
        return self.height;
    }

    pub fn get_bytes_size(&self) -> u32 {
        4 * self.pixels.len() as u32
    }

    // fn save_as_file(&self, filename: &str, bit_depth: BitDepth) -> Result<(), String> {
    //     let file = File::create(self, bit_depth);
    //     use std::error::Error;
    //     use std::io::Write;
    //     let mut bit_stream = file.to_bytes();
    //     let mut file = match std::fs::File::create(filename) {
    //         Err(why) => {
    //             return Err(
    //                 format!("Couldn't create {}: {}", filename, why.description()).to_owned(),
    //             )
    //         }
    //         Ok(file) => file,
    //     };
    //     match file.write_all(bit_stream.as_mut_slice()) {
    //         Err(why) => {
    //             Err(format!("Couldn't write to {}: {}", filename, why.description()).to_owned())
    //         }
    //         Ok(_) => Ok(()),
    //     }
    // }
}