use crate::img::Img;
use super::bit_depth::BitDepth;
use super::bmp_header::BMPHeader;
use super::dib_header::DIBHeader;

/// the struct of .bmp file
pub struct BMPFile {
    header: BMPHeader,
    dib_header: DIBHeader,
    // colors: RgbQuad,
    // data: FileData,
}

impl BMPFile {
    pub fn from_inner(img : &Img,bit_depth: BitDepth) -> BMPFile {
        let dib_header = DIBHeader::from(img,bit_depth);
        let dib_header_size = dib_header.get_byte_size();
        let data_size = img.get_bytes_size();
        let header = BMPHeader::new(data_size, color_size, dib_header_size);
        BMPFile {
            header,
            dib_header,
        }
    }
}