///bmp file head.
///more info: https://en.wikipedia.org/wiki/BMP_file_format
pub struct BMPHeader{
    /// usually use BM – Windows 3.1x, 95, NT, ... etc.
    bitmap_type: [char; 2],
    /// specifies the size of the file in bytes
    size: u32,
    reserved1: u16,
    reserved2: u16,
    /// Specifies the offset from the beginning of the file to the bitmap data.
    offset: u32,
}

impl BMPHeader {
    pub fn get_byte_size() -> usize {
        14
    }
}


impl BMPHeader {
    pub fn new(data_size: u32, color_size: u32, dib_header_size: u32) -> BMPHeader { 
        let offset = color_size + dib_header_size + BMPHeader::get_byte_size() as u32;
        let size = data_size + offset;
        BMPHeader { 
            bitmap_type : ['B', 'M'],
            size, 
            reserved1 : 0, 
            reserved2 : 0, 
            offset,
        } 
    }
}