use image::Rgba;

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