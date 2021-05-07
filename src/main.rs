mod colors;
mod utils;
mod picture;

extern crate image;

use image::{GenericImageView};
use crate::picture::FramedPicture;
use crate::utils::{get_colors_from, get_most_freq};
use structopt::StructOpt;
use std::borrow::Cow;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    copy: std::path::PathBuf,
}

// impl Cli {
//     fn basename(path: &str, sep: char) -> Cow<str> {
//         let mut pieces = path.rsplit(sep);
//         match pieces.next() {
//             Some(p) => p.into(),
//             None => path.into(),
//         }
//     }
// }

fn main() {
    // TODO CLI args
    // let args = Cli::from_args();


    let n = 10u32; // number of colors in palette
    let img = image::open("img/rickmorty.jpg").unwrap();

    println!("Original Dimensions: {}x{}", img.dimensions().0, img.dimensions().1);
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        let colors = get_colors_from(&img);


        let top_n = get_most_freq(&colors, n as usize);

        let mut imgcpy = FramedPicture::new(
            img.width(), img.height(), Some(n));
        let palette = imgcpy.draw_palette(n as u32, &top_n);
        imgcpy.copy_img_into(10, &img);
        imgcpy.stick_piece(&palette);

        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
;

    imgcpy.save_img("img/results/rickmorty.jpg");
    palette.save("img/results/rickmorty_pal.png").unwrap();
    println!("Size of a map: {}", colors.len());
    println!("Framed image dimensions : {}x{}",
             imgcpy.get_dimensions().0, imgcpy.get_dimensions().1);
}
