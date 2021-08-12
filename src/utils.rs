use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
use std::path::PathBuf;
use crate::colors::ColorCount;


/// Grabs the `n` most frequently present elements from the Binary Tree Map
///
/// # Arguments
/// * set - Which is a tree set of ColorCount struct
/// * n - how many most frequent entries to get
///
/// # Return
/// * vector - of n most frequent ColorCount structs
pub fn get_most_freq(set: &HashSet<ColorCount>, n: usize) -> Vec<&ColorCount> {
    let mut heap: BinaryHeap<Reverse<(u32, &ColorCount)>> =
        BinaryHeap::with_capacity(n + 1);

    for c in set.into_iter() {
        match heap.peek() {
            Some(v) =>
                if v.0.1.measure_distance(c) < i32::abs(250) { continue; },
            _ => {}
        }
        heap.push(Reverse((c.count, c)));
        if heap.len() > n {
            heap.pop();
        }
    }
    heap.into_sorted_vec().into_iter()
        .map(|r| r.0.1)
        .collect()
}

/// Generate ColorCount struct and calculates the number of
/// occurrences of each color in a given image each image.
///
/// # Arguments
/// * 'img' - A `DynamicImage` to decompose
///
/// # Returns
/// * Binary Tree Set that contains ColorCount structs
pub fn get_colors_from(img: &DynamicImage) -> HashSet<ColorCount> {
    let mut colors : HashSet<ColorCount> = HashSet::new();

    for i in img.pixels() {
        let c = ColorCount::new(i.2);

        if !colors.contains(&c) {
            colors.insert(c);
        } else {
            match colors.take(&c) {
                Some(mut v) => {
                    v.increment_count();
                    colors.insert(v);
                },
                None => {}
            }
        }
    }
    colors
}

pub trait SaveImage {

    /// Get image buffer reference
    fn get_buffer(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>>;

    /// Saves current buffer as an image.
    /// Image format will be set based on the provided path,
    /// which is expected to include the name of the new file.
    ///
    /// # Argument
    /// * path - full or relative path with new filename and format
    fn save_img(&self, path: &PathBuf) {
        self.get_buffer().save(path).unwrap();
    }
}

/// Enum to select the position of the color palette box
#[derive(PartialEq)]
pub enum Placement {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct BoxShape {
    pub width: u32,
    pub height: u32,
}

impl BoxShape {
    pub fn new(width: u32, height: u32) -> Self {
        BoxShape { width, height }
    }
}

