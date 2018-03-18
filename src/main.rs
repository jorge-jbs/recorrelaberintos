extern crate image;
extern crate cons_list;
extern crate rayon;

use std::fs::File;
use std::collections::HashMap;
use image::ImageDecoder;

mod bfs;
use bfs::breadth_first_search;

mod pbfs;
use pbfs::parallel_breadth_first_search;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pos {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Pos {
    fn from((x, y): (u32, u32)) -> Pos {
        Pos {
            x,
            y,
        }
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Graph {
    start: Pos,
    end: Pos,
    nodes: HashMap<Pos, [Option<Pos>; 4]>,
}

fn main() {
    let graph = read_graph();
    println!("{}", graph.nodes.len());
    println!("");
    println!("{:?}", parallel_breadth_first_search(graph).len());
}

fn read_graph() -> Graph {
    let (buf, width, height) = {
        let mut decoder = image::png::PNGDecoder::new(File::open(format!("examples-from-computerphile/{}.png", ::std::env::args().skip(1).next().expect("Introduce the name of the maze"))).unwrap());
        let (width, height) = decoder.dimensions().unwrap();
        let frames = decoder.into_frames().unwrap().next().unwrap();
        (frames.into_buffer(), width, height)
    };
    let mut start = None;
    let mut end = None;
    let mut nodes = HashMap::new();
    for (x, y, pixel) in buf.enumerate_pixels() {
        if pixel.data[0] == 255 {
            macro_rules! up {
                () => {{
                    let up = (x, (0..y).rev().find(|&y_| nodes.contains_key(&(x, y_).into())).unwrap()).into();
                    nodes.get_mut(&up).unwrap()[1] = Some((x, y).into());
                    up
                }}
            }
            macro_rules! left {
                () => {{
                    let left = ((0..x).rev().find(|&x_| nodes.contains_key(&(x_, y).into())).unwrap(), y).into();
                    nodes.get_mut(&left).unwrap()[2] = Some((x, y).into());
                    left
                }}
            }
            if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
                let north = buf.get_pixel(x, y-1).data[0] == 255;
                let south = buf.get_pixel(x, y+1).data[0] == 255;
                let east = buf.get_pixel(x+1, y).data[0] == 255;
                let west = buf.get_pixel(x-1, y).data[0] == 255;
                match (north, south, east, west) {
                    (false, false, true, true) => (),  // horizontal corridor
                    (true, true, false, false) => (),  // vertical corridor
                    (false, false, false, false) => (),  // lonely
                    (false, _, _, false) => {  // top-left corner
                        nodes.insert((x, y).into(), [None, None, None, None]);
                    }
                    (false, _, _, true) => {  // top-right corner
                        let left = left!();
                        nodes.insert((x, y).into(), [None, None, None, Some(left)]);
                    }
                    (true, _, _, false) => {  // bottom-left corner
                        let up = up!();
                        nodes.insert((x, y).into(), [Some(up), None, None, None]);
                    }
                    (true, _, _, true) => {  // bottom-right corner
                        let up = up!();
                        let left = left!();
                        nodes.insert((x, y).into(), [Some(up), None, None, Some(left)]);
                    }
                }
            } else if y == 0 {
                nodes.insert((x, y).into(), [None, None, None, None]);
                start = Some((x, y).into());
            } else if y == height-1 {
                let up = up!();
                nodes.insert((x, y).into(), [Some(up), None, None, None]);
                end = Some((x, y).into());
            }
        }
    }
    Graph {
        start: start.unwrap(),
        end: end.unwrap(),
        nodes,
    }
}
