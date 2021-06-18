use std::fs;

use image_to_ascii;
fn main() {
    // let ascii_img = image_to_ascii::create::from_image();
    
    let ascii_image = image_to_ascii::create::from_image("src/image_to_ascii/test_images/waltar.png", 125, None, None);
    //println!("{}", ascii_image);
    fs::write("./output", ascii_image).expect("Writing to file failed");
    // TODO: Implement algorithm for correcting vertical stretching due to font aspect ratio not being perfectly square
    // this reduces weirdness a bit. Requires a bit of research, but seems to be around 0.5. Provide a default but allow custom values
    // to apply would have to multiply image aspect ratio with fornt aspect ratio first and resize, then resize to desired format

    // TODO: build a simple CLI
    // TODO: create library for wasm, try out in react app
    // TODO: gif support if I feel like it, would like a way to export it as a gif of the text or as single frames in text form
    // DONE: error handling
    // TODO: DOCS
    // ascii_img.save("./buttercat.txt");
}
