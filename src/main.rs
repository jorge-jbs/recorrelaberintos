extern crate image;

use std::fs::File;
use std::rc::Rc;
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
    let mut decoder = image::bmp::BMPDecoder::new(File::open("laberinto-1.bmp").unwrap());
    let (width, height) = decoder.dimensions().unwrap();
    let frames = decoder.into_frames().unwrap().next().unwrap();
    let buf = frames.into_buffer();
    let mut nodes: Vec<Rc<Node>> = Vec::new();
    let mut start = None;
    let mut end = None;
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
                    nodes.push(Rc::new(Node {
                        pos: (x, y),
                        node_type: if n == 2 { NodeType::Joint } else { NodeType::Corner },
                        siblings: Vec::new(),
                    }));
                }
            } else {
                if y == 0 {
                    let node = Rc::new(Node {
                        pos: (x, y),
                        node_type: NodeType::Start,
                        siblings: Vec::new(),
                    });
                    nodes.push(Rc::clone(&node));
                    start = Some(node);
                } else if y == height-1 {
                    let node = Rc::new(Node {
                        pos: (x, y),
                        node_type: NodeType::End,
                        siblings: Vec::new(),
                    });
                    nodes.push(Rc::clone(&node));
                    end = Some(node);
                }
            }
        }
    }
    let nodes = nodes;
    let start = start.unwrap();
    let end = end.unwrap();

    nodes.iter().inspect(|x| println!("{:?} - {:?}", x.pos, x.node_type)).collect::<Vec<_>>();
}
