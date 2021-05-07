mod colors;
mod utils;
mod img_processor;

extern crate image;

use std::collections::{BinaryHeap, BTreeSet};
use image::{GenericImageView, Rgba, DynamicImage, ImageBuffer};
use std::cmp::Reverse;
use crate::colors::ColorCount;
use crate::img_processor::{compute_palette_size, copy_img_into, draw_palette};
use image::imageops;

fn main() {
    let n = 10; // number of colors in palette
    let path = String::from("/home/bexx/Projects/paleatra/img/akira.jpg");
    let img1 = image::open(path).unwrap();

    println!("Dimensions: {}x{}", img1.dimensions().0, img1.dimensions().1);
    let colors = get_colors_from(&img1);


    let top_n = get_most_freq(&colors, n);


    // NOTE: 1. Create copy of an image with white frame around it
    let dims = compute_palette_size(&img1.dimensions(), 10);
    let w = img1.dimensions().0 + 20;
    let h = img1.dimensions().1 + 30 + dims.0;
    let mut imgcpy: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(
        w, h, |_,_| { Rgba([255, 252, 234, 1]) }
    );
    copy_img_into(&mut imgcpy, 10, &img1);


    // NOTE: 2. create a pallete image with yellowish background and top 10 colors
    let mut palette = draw_palette(dims, n as u32, &top_n);

    let mut xp = img1.dimensions().0 + 10;
    let mut yp = img1.dimensions().1 + 10;
    // let mut y= 0;
    imageops::overlay(
        &mut imgcpy, &mut palette, 10, img1.height() + 20);

    imgcpy.save("copy.jpg").unwrap();
    palette.save("result.jpg").unwrap();

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

