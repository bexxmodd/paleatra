extern crate image;

use std::collections::{BTreeMap, BinaryHeap};
use image::{GenericImageView, Rgba, DynamicImage};
use std::cmp::{Reverse, Eq, Ord};
use std::hash::Hash;

fn main() {
    let path = String::from("/home/bexx/Projects/paleatra/img/test.jpg");
    let img1 = image::open(path).unwrap();

    println!("Dimensions: {}x{}", img1.dimensions().0, img1.dimensions().1);
    let colors = get_colors_from(&img1);

    let mut top_ten = get_most_freq(&colors, 10);

    let mut i = 1;
    for x in top_ten {
        println!("{}. {} - {}", i, x.0, x.1);
        i += 1;
    }

    // TODO: 1. create a new square image with each color frop top list and hex code

    // TODO: 2. append colored squares to the right if image is vertical, bottom if horizontal

    println!("Size of a map: {}", colors.len());
}

/// Grabs the n most frequently present elements from the Binary Tree Map
///
/// # Arguments
/// map - Which is a treemap where values are the count of key element
/// n - how many most frequent entries to get
///
/// # Return
/// vector - of n most frequent values
pub fn get_most_freq<K, V>(map: &BTreeMap<K, V>, n: usize) -> Vec<(&V, &K)>
    where
        K: Hash + Eq + Ord,
        V: Eq + Ord {
    let mut heap = BinaryHeap::with_capacity(n + 1);
    for (x, count) in map.into_iter() {
        heap.push(Reverse((count, x)));
        if heap.len() > n {
            heap.pop();
        }
    }
    heap.into_sorted_vec().into_iter()
        .map(|r| r.0).collect()
}

/// Gets the count of the all color pixels from the image
///
/// # Arguments
/// * 'img' - A DynamicImage to decompose
///
/// # Returns
/// * Binary Tree Map with hex code of the color as key
///          and count for that color as value
pub fn get_colors_from(img: &DynamicImage) -> BTreeMap<String, i32> {
    let mut colors = BTreeMap::new();

    for i in img.pixels() {
        *colors.entry(get_hex_of(&i.2))
            .or_insert(0) += 1;
    }
    colors
}

/// Converts the RGBA struct which represents color into a hex code
///
/// # Arguments
/// color - RGBA struct
///
/// # Return
/// hex code - of the color as a String
pub fn get_hex_of(color: &Rgba<u8>) -> String {
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