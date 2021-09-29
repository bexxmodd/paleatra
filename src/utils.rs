use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread
};
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
                if v.0.1.measure_distance(c) < i32::abs(250) {
                    continue
                },
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
    let colors = Arc::new(Mutex::new(HashSet::new()));
    let tmp: Arc<Mutex<Vec<_>>> = Arc::new(Mutex::new(img.pixels().collect()));
    let mut handles = vec![];

    let slice = tmp.lock().unwrap().len() / 5;
    let mut start = 0usize - slice;
    let mut finish = 0;

    for _ in 0..5 {
        start += slice;
        finish += slice;
        let colors = Arc::clone(&colors);
        let tmp = Arc::clone(&tmp);
        let handle = thread::spawn(move || {
            for i in &tmp.lock().unwrap()[start..finish] {
                let c = ColorCount::new(i.2);
                let mut col_set = colors.lock().unwrap();

                if !col_set.contains(&c) {
                    col_set.insert(c);
                } else {
                    match col_set.take(&c) {
                        Some(mut v) => {
                            v.increment_count();
                            col_set.insert(v);
                        },
                        None => {}
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let res  = *colors.lock().unwrap();
    res
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
    pub fn new<TW, TH>(w: TW, h: TH) -> Self
        where
            TW: Into<u32>,
            TH: Into<u32>
    {
        let width = w.into();
        let height = h.into();
        BoxShape { width, height }
    }
}

