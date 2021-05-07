mod colors;
mod utils;
mod img_copy;

extern crate image;

use std::collections::{BinaryHeap, BTreeSet};
use image::{GenericImageView, DynamicImage};
use std::cmp::Reverse;
use crate::colors::ColorCount;
use crate::img_copy::FramedPicture;

fn main() {
    let n = 10u32; // number of colors in palette
    let path = String::from(
        "/home/bexx/Projects/paleatra/img/rickmorty.jpg");
    let img1 = image::open(path).unwrap();

    println!("Original Dimensions: {}x{}", img1.dimensions().0, img1.dimensions().1);
    let colors = get_colors_from(&img1);


    let top_n = get_most_freq(&colors, n as usize);

    let mut imgcpy = FramedPicture::new(
        img1.width(), img1.height());
    let mut palette = imgcpy.draw_palette(n as u32, &top_n);
    imgcpy.copy_img_into(10, &img1);
    imgcpy.stick_piece(&palette);

    imgcpy.save_img("result.jpg");
    palette.save("pal.jpg").unwrap();
    // println!("After editing: {}x{}", )
    println!("Size of a map: {}", colors.len());
}

/// Grabs the n most frequently present elements from the Binary Tree Map
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

/// Generate ColorCount struct with count from each image
///
/// # Arguments
/// * 'img' - A DynamicImage to decompose
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

