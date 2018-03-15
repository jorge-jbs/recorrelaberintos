extern crate image;

use std::fs::File;
use std::rc::Rc;
use std::collections::HashMap;
use image::ImageDecoder;

#[derive(Clone, Copy, Debug)]
enum NodeType {
    Start,  // node with one sibling at the top of the image
    End,  // node with one sibling at the bottom of the image
    Joint,  // node with two siblings
    Corner,  // node with more than two siblings
}

struct Node {
    pos: (u32, u32),
    node_type: NodeType,
    siblings: Vec<Rc<Node>>,
}

fn main() {
    let nodes = read_nodes();
    nodes.iter().inspect(|&(pos, node_type)| println!("{:?} - {:?}", pos, node_type)).collect::<Vec<_>>();
}

fn read_nodes() -> HashMap<(u32, u32), NodeType> {
    let mut decoder = image::bmp::BMPDecoder::new(File::open("laberinto-1.bmp").unwrap());
    let (width, height) = decoder.dimensions().unwrap();
    let frames = decoder.into_frames().unwrap().next().unwrap();
    let buf = frames.into_buffer();
    let mut nodes: HashMap<(u32, u32), NodeType> = HashMap::new();
    for (x, y, pixel) in buf.enumerate_pixels() {
        if pixel.data[0] == 255 {
            if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
                let mut n = 0;
                let mut north = false;
                let mut south = false;
                let mut east = false;
                let mut west = false;
                if buf.get_pixel(x-1, y).data[0] == 255 {
                    west = true;
                    n += 1;
                }
                if buf.get_pixel(x+1, y).data[0] == 255 {
                    east = true;
                    n += 1;
                }
                if buf.get_pixel(x, y-1).data[0] == 255 {
                    north = true;
                    n += 1;
                }
                if buf.get_pixel(x, y+1).data[0] == 255 {
                    south = true;
                    n += 1;
                }
                if n > 2 || !((north && south) || (east && west)) {
                    nodes.insert((x, y), if n == 2 { NodeType::Joint } else { NodeType::Corner });
                }
            } else {
                if y == 0 {
                    nodes.insert((x, y), NodeType::Start);
                } else if y == height-1 {
                    nodes.insert((x, y), NodeType::End);
                }
            }
        }
    }
    nodes
}
