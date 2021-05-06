extern crate image;

use std::collections::{BTreeMap, BinaryHeap, BTreeSet};
use image::{GenericImageView, Rgba, DynamicImage, ImageBuffer};
use std::cmp::{Reverse, Eq, Ord, Ordering};
use std::hash::Hash;
// use std::intrinsics::variant_count;
use std::fmt;

fn main() {
    let path = String::from("/home/bexx/Projects/paleatra/img/test.jpg");
    let img1 = image::open(path).unwrap();

    println!("Dimensions: {}x{}", img1.dimensions().0, img1.dimensions().1);
    let colors = get_colors_from(&img1);

    let mut top_ten = get_most_freq(&colors, 10);

    let mut i = 1;
    for x in top_ten {
        println!("{}. {} - {}", i, x.1.hex, x.0);
        i += 1;
    }

    // TODO: 1. create a new square image with each color frop top list and hex code
    // let mut img = ImageBuffer::new(100, 100);
    // for (x, y, pix) in img.enumerate_pixels_mut() {
    //     *pix = image::Rgba([10, 10, 5, 255]);
    // }

    // TODO: 2. append colored squares to the right if image is vertical, bottom if horizontal

    println!("Size of a map: {}", colors.len());
}

pub struct PalColor {
    rgba: Rgba<u8>,
    hex: String,
    count: u32,
}

impl PalColor {
    pub fn new(rgba: Rgba<u8>) -> PalColor {
        PalColor {
            rgba,
            hex: generate_hex(&rgba),
            count: 1
        }
    }

    pub fn increment_count(&mut self) {
        self.count += 1;
    }
}

impl Ord for PalColor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hex.cmp(&other.hex)
    }
}

impl PartialEq for PalColor {
    fn eq(&self, other: &Self) -> bool {
        self.hex == other.hex
    }
}

impl PartialOrd for PalColor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PalColor {}


/// Grabs the n most frequently present elements from the Binary Tree Map
///
/// # Arguments
/// set - Which is a tree set of PalColor struct
/// n - how many most frequent entries to get
///
/// # Return
/// vector - of n most frequent PalColor structs
pub fn get_most_freq(set: &BTreeSet<PalColor>, n: usize) -> Vec<(u32, &PalColor)> {
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

/// Gets the count of the all color pixels from the image
///
/// # Arguments
/// * 'img' - A DynamicImage to decompose
///
/// # Returns
/// * Binary Tree Set that contains PalColor structs
pub fn get_colors_from(img: &DynamicImage) -> BTreeSet<PalColor> {
    let mut colors : BTreeSet<PalColor> = BTreeSet::new();

    for i in img.pixels() {
        let c = PalColor::new(i.2);

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

/// Converts the RGBA struct which represents color into a hex code
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