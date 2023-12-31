mod bmp_header;
mod dib_header;
mod bmp_file;
pub mod bit_depth;
mod color_table;

use crate::{img::Img, bmp::bmp_file::BMPFile};
use self::bit_depth::BitDepth;
use std::io::Write;

pub fn save_as_file(img:&Img, filename: &str, bit_depth: BitDepth) -> Result<(), String> {
    let file = BMPFile::from_inner_img(img, bit_depth);
    let mut bit_stream = file.to_bytes();

    let save_path = std::path::Path::new(filename);
    std::fs::create_dir_all(save_path.parent().unwrap()).unwrap();
    let mut file = match std::fs::File::create(save_path) {
        Err(why) => {
            return Err(
                format!("Couldn't create {}: {}", filename, why.to_string()).to_owned(),
            )
        }
        Ok(file) => file,
    };
    match file.write_all(bit_stream.as_mut_slice()) {
        Err(why) => {
            Err(format!("Couldn't write to {}: {}", filename, why.to_string()).to_owned())
        }
        Ok(_) => Ok(()),
    }
}

#[cfg(test)]
mod test {
    use crate::{img::Img, color::Color};
    use super::bit_depth::BitDepth;

    #[test]
    fn try_to_write_img() {
        let mut img = Img::new(255, 255);
        for x in 0..255 {
            for y in 0..255 {
                let color = Color::new(x as u8, y as u8, x as u8, 255);
                match img.set_pixel(x, y, color) {
                    Err(msg) => panic!("{}",msg),
                    _ => (),
                };
            }
        }
        let save_result = super::save_as_file(&img,"../test_out/test.bmp",BitDepth::AllColors);
        match save_result {
            Err(msg) => panic!("{}",msg),
            _ => (),
        };
    }
}