use image::{Rgba, DynamicImage, GenericImageView};
use std::collections::{BTreeSet, BinaryHeap};
use crate::colors::ColorCount;
use std::cmp::Reverse;


/// Grabs the `n` most frequently present elements from the Binary Tree Map
///
/// # Arguments
/// * set - Which is a tree set of ColorCount struct
/// * n - how many most frequent entries to get
///
/// # Return
/// * vector - of n most frequent ColorCount structs
pub fn get_most_freq(set: &BTreeSet<ColorCount>, n: usize) -> Vec<(u32, &ColorCount)> {
    let mut heap = BinaryHeap::with_capacity(n + 1);
    for c in set.into_iter() {
        heap.push(Reverse((c.count, c)));
        if heap.len() > n {
            heap.pop();
        }
    }
    heap.into_sorted_vec().into_iter()
        .map(|r| r.0).collect()
}

/// Generate ColorCount struct and calculates the number of
/// occurrences of each color in a given image each image.
///
/// # Arguments
/// * 'img' - A `DynamicImage` to decompose
///
/// # Returns
/// * Binary Tree Set that contains ColorCount structs
pub fn get_colors_from(img: &DynamicImage) -> BTreeSet<ColorCount> {
    let mut colors : BTreeSet<ColorCount> = BTreeSet::new();

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

/// Converts the RGBA tuple of the color into a hex code string
/// hex is all upper case and starts with 'x'.
///     example view: `x2F3FB6`
///
/// # Arguments
/// color - RGBA struct
///
/// # Return
/// hex code - of the color as a String
pub fn generate_hex(color: &Rgba<u8>) -> String {
    let mut hexcode = "x".to_owned();

    let red = format!("{:X}", color[0]);
    let green = format!("{:X}", color[1]);
    let blue = format!("{:X}", color[2]);
    let alpha = format!("{:X}", color[3]);

    hexcode.push_str(&*red);
    hexcode.push_str(&*green);
    hexcode.push_str(&*blue);
    hexcode.push_str(&*alpha);

    hexcode
}