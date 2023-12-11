use crate::img::Img;

use super::bit_depth::BitDepth;


///Common support version, without OS/2
pub struct DIBHeader{
    /// specifies the size of the BitMapFileHeader structure, in bytes, as now it's 40
    size: u32,
    /// specifies the width of the image, in pixels
    width: u32,
    /// specifies the height of the image, in pixels
    height: u32,
    /// specifies the number of planes of the target device, must be set to one
    planes: u16,
    /// specifies the number of bits per pixel
    /// possible values are as follows:
    ///  - 1 (black / white)
    ///  - 4 (16 colors)
    ///  - 8 (256 colors)
    ///  - 24 (16.7 million colors)
    bit_depth: u16,
    /// specifies the type of compression, usually set to zero (no compression)
    compression: u32,
    /// specifies the size of the image data, in bytes. If there is no
    /// compression, it is valid to set this member to zero
    size_image: u32,
    /// specifies the the horizontal pixels per meter on the designated target
    /// device, usually set to zero.
    x_pixels_per_meter: u32,
    /// specifies the vertical pixels per meter on the designated target device,
    /// usually set to zero
    y_pixels_per_meter: u32,
    /// specifies the number of colors used in the bitmap, if set to zero the
    /// number of colors is calculated using the biBitDepth member.
    colors_used: u32,
    /// specifies the number of color that are 'important' for the bitmap, if set
    /// to zero, all colors are important
    colors_important: u32,
}

impl DIBHeader {
    pub fn from(img : &Img, bit_depth: BitDepth) -> DIBHeader {
        DIBHeader {
            size: 40,
            width: img.get_width(),
            height: img.get_height(),
            bit_depth: bit_depth as u16,
            planes: 1,
            compression: 0,
            size_image: 0,
            x_pixels_per_meter: 0,
            y_pixels_per_meter: 0,
            colors_used: 0,
            colors_important: 0,
        }
    }

    pub fn get_byte_size(&self) -> u32 {
        self.size
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(&self.width.to_le_bytes());
        bytes.extend_from_slice(&self.height.to_le_bytes());
        bytes.extend_from_slice(&self.planes.to_le_bytes());
        bytes.extend_from_slice(&self.bit_depth.to_le_bytes());
        bytes.extend_from_slice(&self.compression.to_le_bytes());
        bytes.extend_from_slice(&self.size_image.to_le_bytes());
        bytes.extend_from_slice(&self.x_pixels_per_meter.to_le_bytes());
        bytes.extend_from_slice(&self.y_pixels_per_meter.to_le_bytes());
        bytes.extend_from_slice(&self.colors_used.to_le_bytes());
        bytes.extend_from_slice(&self.colors_important.to_le_bytes());
        bytes
    }
}