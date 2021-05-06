mod colors;
mod utils;

extern crate image;

use std::collections::{BinaryHeap, BTreeSet};
use image::{GenericImageView, Rgba, DynamicImage, ImageBuffer};
use std::cmp::Reverse;
use crate::colors::ColorCount;

fn main() {
    let n = 10; // number of colors in palette
    let path = String::from("/home/bexx/Projects/paleatra/img/ibra.jpg");
    let img1 = image::open(path).unwrap();

    println!("Dimensions: {}x{}", img1.dimensions().0, img1.dimensions().1);
    let colors = get_colors_from(&img1);
    // let mut img_cpy = ImageBuffer::new(img1.dimensions().0 + 20, img1.dimensions().1 + 20);

    let top_n = get_most_freq(&colors, n);

    // NOTE: create a pallete image with yellowish background and top 10 colors
    let dims = compute_palette_size(&img1.dimensions());
    let palette = create_palette(dims, n as u32, &top_n);
    palette.save("result.jpg").unwrap();


    // TODO: 2. append colored squares to the right if image is vertical, bottom if horizontal

    println!("Size of a map: {}", colors.len());
}

pub fn create_palette(dims: (u32, u32), n: u32,
                      top_colors: &Vec<(u32, &ColorCount)>)
    -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let pal_width = dims.0 * n + 110;
    let pal_height = dims.1 + 10;

    let mut palette = ImageBuffer::from_fn(
        pal_width, pal_height, |_,_| { Rgba([255, 252, 234, 1]) }
    );

    let mut xp = 0;
    for color in top_colors {
        xp += 10; // skip the frame vertical line
        for _ in 0..dims.0 {
            let mut yp = 0;
            while yp < dims.1 {
                palette.put_pixel(xp, yp, color.1.rgba);
                yp += 1;
            }
            xp += 1;
        }
    }
    palette
}

pub fn compute_palette_size(img_dims: &(u32, u32)) -> (u32, u32) {
    let size: u32;
    if img_dims.0 > img_dims.1 {
        size = img_dims.0 / 10;
    } else {
        size = img_dims.1 / 10;
    }
    (size, size)
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

