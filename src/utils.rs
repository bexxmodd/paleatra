use image::{DynamicImage, GenericImageView};
use std::collections::{BinaryHeap, HashSet};
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
pub fn get_most_freq(set: &HashSet<ColorCount>, n: usize) -> Vec<(u32, &ColorCount)> {
    let mut heap: BinaryHeap<Reverse<(u32, &ColorCount)>>  = BinaryHeap::with_capacity(n + 1);
    for c in set.into_iter() {
        match heap.peek() {
            Some(v) =>
                if v.0.1.measure_diff(c) < i32::abs(250) { continue; },
            _ => {}
        }
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

