extern crate image;

use std::fs::File;
use std::rc::Rc;
use image::ImageDecoder;

struct Node {
    pos: (u32, u32),
    siblings: Vec<Rc<Node>>,
}

fn main() {
    let mut decoder = image::bmp::BMPDecoder::new(File::open("laberinto-1.bmp").unwrap());
    let frames = decoder.into_frames().unwrap().next().unwrap();
    let buf = frames.into_buffer();
    let mut dy = 0;
    for (x, y, pixel) in buf.enumerate_pixels() {
        if dy != y {
            println!("");
        }
        dy = y;
        print!("{:3}  ", pixel.data[0]);
    }
}
