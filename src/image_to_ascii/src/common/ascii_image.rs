use image::{self};
/// Represents an image in ascii
#[derive(Debug)]
pub struct AsciiImage {
    /// represents the width of the image
    width: u32,
    /// represents the height of the image
    height: u32,
    /// holds letters as pixels. Line breaks are calculated from the width of the image
    pixels: Vec<AsciiPixel>,
}

impl AsciiImage {
    /// Creates new instance of AsciiImage
    /// p represents brightness level of a grayscale image, generates letters from these values
    pub fn new(w: u32, h: u32, p: Vec<u8>) -> AsciiImage {
        let mut new_vec: Vec<AsciiPixel> = vec![];

        for pixel in p.iter() {
            new_vec.push(AsciiPixel::new(*pixel));
        }

        AsciiImage {
            width: w,
            height: h,
            pixels: new_vec,
        }
    }

    pub fn create_image(&self, brightness_levels: Option<Vec<char>>) -> String {
        let mut chars: Vec<char> = vec![' '; self.width as usize];

        for (idx, pixel) in self.pixels.iter().enumerate() {
            if idx as u32 % self.width == 0 {
                chars.push('\n');
            }
            chars.push(pixel.get_letter(&brightness_levels));
        }

        let s: String = chars.iter().collect();
        return s;
    }
}

#[derive(Debug)]
struct AsciiPixel {
    /// brightness from 0 - 255
    brightness: u8,
}

impl AsciiPixel {
    pub fn new(b: u8) -> AsciiPixel {
        AsciiPixel { brightness: b }
    }

    /// returns the letter corresponding to the brightness
    pub fn get_letter(&self, brightness_levels: &Option<Vec<char>>) -> char {
        let mut letter: char = ' ';
        let b = normalize_brightness(self.brightness);

        let brightness_levels: Vec<char> = match brightness_levels {
            None => vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
            Some(levels) => levels.to_vec(),
        };

        for (idx, bl) in brightness_levels.iter().enumerate() {
            if b >= idx as f32 / brightness_levels.len() as f32 {
                letter = *bl;
            } else {
                break;
            }
        }
        return letter;
    }
}

fn normalize_brightness(b: u8) -> f32 {
    return b as f32 / 255 as f32;
}

pub fn create_ascii_image(img: &image::DynamicImage) -> AsciiImage {
    let img = img.as_luma8().expect("Converting image to luma failed");

    let mut pixels: Vec<u8> = Vec::new();
    for pixel in img.as_raw().iter() {
        pixels.push(*pixel);
    }

    match img.save("new.jpg") {
        Err(e) => {panic!("{:?}", e);},
        Ok(res) => res
    };

    return AsciiImage::new(img.width(), img.height(), pixels);
}
