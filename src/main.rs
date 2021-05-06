extern crate image;

use std::collections::{HashMap, BTreeSet, BTreeMap};
use image::{GenericImageView, Rgba, DynamicImage};

struct FrequentColor {
    hex: String,
    count: u16,
}

fn build_color(hex: String, count: u16) -> FrequentColor {
    FrequentColor {hex, count}
}

fn main() {
    let path = String::from("/home/bexx/Projects/paleatra/img/test2.jpg");
    let img1 = image::open(path).unwrap();
    let colors = get_colors_from(&img1);

    // let mut top_ten = get_most_freq(&colors, 10);

    let mut i = 1;
    for x in &colors {
        println!("{}. {} - {}", i, x.0, x.1);
        i += 1;
    }

    // let mut i = 1;
    // for x in top_ten {
    //     println!("{}. {}", i, x);
    //     i += 1;
    // }

    println!("Size of a map: {}", colors.len());
}

pub fn get_most_freq<K, V>(map: &HashMap<K, V>, n: u16) -> Vec<K> {
    let mut top = vec![];
    top
}

pub fn get_colors_from(img: &DynamicImage) -> BTreeMap<String, i32> {
    let mut colors = BTreeMap::new();

    for i in img.pixels() {
        *colors.entry(get_hex_of(&i.2))
            .or_insert(0) += 1;
    }
    colors
}

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