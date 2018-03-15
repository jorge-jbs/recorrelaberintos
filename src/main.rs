extern crate image;

use std::fs::File;
use image::ImageDecoder;

fn main() {
    let mut decoder = image::bmp::BMPDecoder::new(File::open("laberinto-1.bmp").unwrap());
    if let image::DecodingResult::U8(vec) = decoder.read_image().unwrap() {
        for 
        for p in vec {
            print!("{:3} ", p);
        }
    }
    println!("{:?}", decoder.colortype());
}
