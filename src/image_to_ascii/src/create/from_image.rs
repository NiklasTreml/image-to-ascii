use crate::common::ascii_image::create_ascii_image;
use image::{self, GenericImageView, imageops};

/// Creates a string containing an ascii representation of the passed image
/// Takes the path to the image as &str
/// ascii_width and ascii_height represents the height of the outputted string in characters. The height can be omitted, if so it will be calculated from the width using the aspect ratio of the source image 
/// brightness_levels is used for the brightness of the pixels. should be sorted from dark to bright. Reverse to get a negative 

pub fn from_image(
    path_to_image: &str,
    ascii_width: u32,
    ascii_height: Option<u32>,
    brightness_levels: Option<Vec<char>>,
) -> String {
    println!("from image");
    let img = match image::open(path_to_image){
        Ok(res) => res,
        Err(e)=> panic!("Could not read image:{:?}", e)
    };

    let img = img.grayscale();
    // let new_width = ((img.as_luma8().unwrap().width() as f32) * wscale_factor) as u32;
    let mut img = img.resize_exact(img.width(), (img.height() as f32 * 0.5) as u32, imageops::Lanczos3);

    match ascii_height {
        Some(h) => {
            img = img.resize_exact(ascii_width, h, imageops::Lanczos3);
        },
        None => {
            img = img.resize(ascii_width, ascii_width, imageops::Lanczos3);

        }
    };
    

    let ascii_img = create_ascii_image(&img);

    // println!("{:?}", ascii_img);

    return ascii_img.create_image(brightness_levels);
    // return AsciiImage::New();
}
