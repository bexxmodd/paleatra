mod colors;
mod utils;
mod picture;

extern crate image;

use image::{GenericImageView};
use crate::picture::FramedPicture;
use crate::utils::{get_colors_from, get_most_freq};

fn main() {
    let n = 10u32; // number of colors in palette
    let path = String::from(
        "/home/bexx/Projects/paleatra/img/ghost2.jpg");
    let img1 = image::open(path).unwrap();

    println!("Original Dimensions: {}x{}", img1.dimensions().0, img1.dimensions().1);
    let colors = get_colors_from(&img1);


    let top_n = get_most_freq(&colors, n as usize);

    let mut imgcpy = FramedPicture::new(
        img1.width(), img1.height(), Some(n));
    let palette = imgcpy.draw_palette(n as u32, &top_n);
    imgcpy.copy_img_into(10, &img1);
    imgcpy.stick_piece(&palette);

    imgcpy.save_img("img/results/ghost2.jpg");
    palette.save("img/results/ghost2_pal.png").unwrap();
    println!("Size of a map: {}", colors.len());
    println!("Framed image dimensions : {}x{}",
             imgcpy.get_dimensions().0, imgcpy.get_dimensions().1);
}
