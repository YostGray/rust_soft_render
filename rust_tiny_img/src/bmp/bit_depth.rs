#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BitDepth {
    Color2Bit = 1,
    Color16Bit = 4,
    Color256Bit = 8,
    AllColors = 24,
    AllColorsAndShades = 32,
}

impl BitDepth {
    ///
    /// Get the number of bits or bytes to read when trying
    /// to read in a file with a specified bit depth value
    ///
    /// For the ColorNBit, we return the total length of how much each color
    /// takes up in """bits"""
    ///
    /// For AllColors*, we return the total length of how much each color takes
    /// up in """bytes"""
    ///
    pub fn get_step_counter(&self) -> u32 {
        match self {
            Self::Color2Bit => 1,
            Self::Color16Bit => 4,
            Self::Color256Bit => 8,
            Self::AllColors => 3,
            Self::AllColorsAndShades => 4,
        }
    }

    // ///
    // /// Get a suggested bit depth depending on the colors contained inside of
    // /// a array of colors
    // ///
    // pub fn get_suggested_bit_depth(bitmap: &BitMap) -> BitDepth {
    //     let unique_colors = bitmap.get_all_unique_colors().len();
    //     let contains_transparents = bitmap.is_image_transparent();
    //     match unique_colors {
    //         0..=2 => BitDepth::Color2Bit,
    //         3..=16 => BitDepth::Color16Bit,
    //         17..=256 => BitDepth::Color256Bit,
    //         _ => match contains_transparents {
    //             true => BitDepth::AllColorsAndShades,
    //             false => BitDepth::AllColors,
    //         },
    //     }
    // }
}
