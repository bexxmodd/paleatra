use image::Rgba;
use std::cmp::Ordering;
use crate::utils;

/// Struct to keep track of number of occurrences of each color
/// Contains the RGBA and hex code of the color
pub struct ColorCount {
    pub rgba: Rgba<u8>,
    pub hex: String,
    pub count: u32,
}

impl ColorCount {
    /// Constructor which generates hex code of
    /// the color from RGBA and sets count to 1
    pub fn new(rgba: Rgba<u8>) -> ColorCount {
        ColorCount {
            rgba,
            hex: utils::generate_hex(&rgba),
            count: 1
        }
    }

    /// Increment the color's count by one
    pub fn increment_count(&mut self) {
        self.count += 1;
    }
}

impl Ord for ColorCount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hex.cmp(&other.hex)
    }
}

impl PartialEq for ColorCount {
    fn eq(&self, other: &Self) -> bool {
        self.hex == other.hex
    }
}

impl PartialOrd for ColorCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ColorCount {}