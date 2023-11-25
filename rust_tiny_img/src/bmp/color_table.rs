use crate::{color::Color, img::Img};

use super::bit_depth::BitDepth;

pub struct ColorTable {
    data: Vec<Color>,
}

impl ColorTable {
    pub fn from(img : &Img, bit_depth: BitDepth) -> ColorTable{
        match bit_depth {
            // BitDepth::Color2Bit | BitDepth::Color16Bit | BitDepth::Color256Bit => RgbQuad {
            //     data: bitmap.get_all_unique_colors(),
            // },
            _ => ColorTable::empty(),
        }
    }

    fn empty() -> ColorTable {
        ColorTable { data: Vec::new() }
    }

    pub fn get_bytes_size(&self) -> u32 {
        4 * self.data.len() as u32
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for color in &self.data {
            bytes.push(color.b());
            bytes.push(color.g());
            bytes.push(color.r());
            bytes.push(color.a());
        }
        bytes
    }
}