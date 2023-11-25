use crate::img::Img;
use super::bit_depth::BitDepth;
use super::bmp_header::BMPHeader;
use super::color_table::ColorTable;
use super::dib_header::DIBHeader;

/// the struct of .bmp file
pub struct BMPFile {
    header: BMPHeader,
    dib_header: DIBHeader,
    color_table: ColorTable,
    img: Img,
}

impl BMPFile {
    pub fn from_inner(img : &Img,bit_depth: BitDepth) -> BMPFile {
        let dib_header = DIBHeader::from(img,bit_depth);
        let dib_header_size = dib_header.get_byte_size();

        let color_table = ColorTable::from(img,bit_depth);
        let color_table_size = color_table.get_bytes_size();

        let data_size = img.get_bytes_size();
        let header = BMPHeader::new(data_size, color_table_size, dib_header_size);
        BMPFile {
            header,
            dib_header,
            color_table,
            img: img.clone(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.header.as_bytes());
        bytes.append(&mut self.dib_header.as_bytes());
        bytes.append(&mut self.color_table.as_bytes());
        bytes.append(&mut self.img.as_bmp_data_bytes());
        bytes
    }
}


impl Img {
    fn as_bmp_data_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut counter = self.get_width();
        for p in self.get_pixels() {
            // i need to watch the row count
            bytes.push(p.b());
            bytes.push(p.g());
            bytes.push(p.r());
            // if self.bit_depth == BitDepth::AllColorsAndShades {
            //     bytes.push(p.get_alpha())
            // }
            // after row has been written, pad the bytes to a number divisible by 4
            counter = counter - 1;
            if counter == 0 {
                while bytes.len() % 4 != 0 {
                    bytes.push(0);
                }
                counter = self.get_width();
            }
        }
        bytes
    }
}