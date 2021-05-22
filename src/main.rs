extern crate image;

use structopt::StructOpt;
use image::{GenericImageView};
use paleatra::utils::{get_colors_from, get_most_freq, SaveImage, Placement};
use paleatra::picture::FramedPicture;

/// simple CLI which holds terminal arguments
#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    copy: std::path::PathBuf,
}


fn main() {
    let args = Cli::from_args();

    let n = 10u32; // number of colors in palette
    let img = image::open(&args.path).unwrap();

    println!("Original image dimensions: {}x{}", img.dimensions().0, img.dimensions().1);
    let colors = get_colors_from(&img);

    let top_n = get_most_freq(&colors, n as usize);

    let place = Placement::Bottom;
    let mut imgcpy = FramedPicture::new(
    img.width(), img.height(), Some(n), place);
    imgcpy.fill_in_palette(&top_n);
    imgcpy.copy_img_into(n, &img);
    imgcpy.combine_pieces();

    imgcpy.save_img(&args.copy);
    // palette.save(&args.copy.set_file_name("palette.png")).unwrap();

    println!("Total unique pixel colors: {}", colors.len());
    println!("Framed image dimensions : {}x{}",
             imgcpy.get_dimensions().0, imgcpy.get_dimensions().1);
}