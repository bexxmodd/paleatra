mod colors;
mod utils;
mod picture;

extern crate image;

use image::{GenericImageView};
use crate::picture::FramedPicture;
use crate::utils::{get_colors_from, get_most_freq};
use structopt::StructOpt;

/// simple CLI which holds terminal arguments
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    copy: std::path::PathBuf,
}

fn main() {
    // TODO CLI args
    let args = Cli::from_args();

    let n = 10u32; // number of colors in palette
    let img = image::open(&args.path).unwrap();

    println!("Original Dimensions: {}x{}", img.dimensions().0, img.dimensions().1);
    let colors = get_colors_from(&img);

    let top_n = get_most_freq(&colors, n as usize);

    let mut imgcpy = FramedPicture::new(
        img.width(), img.height(), Some(n));
    let palette = imgcpy.draw_palette(n as u32, &top_n);
    imgcpy.copy_img_into(10, &img);
    imgcpy.stick_piece(&palette);

    imgcpy.save_img(&args.copy);
    // palette.save(&args.copy.set_file_name("palette.png")).unwrap();

    println!("Total unique pixels: {}", colors.len());
    println!("Framed image dimensions : {}x{}",
             imgcpy.get_dimensions().0, imgcpy.get_dimensions().1);
}
