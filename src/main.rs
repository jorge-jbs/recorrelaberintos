extern crate image;

use std::fs::File;
use std::collections::HashSet;
use image::ImageDecoder;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
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

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        if self.y > other.y {
            Some(std::cmp::Ordering::Greater)
        } else if self.y < other.y {
            Some(std::cmp::Ordering::Less)
        } else {  // self.y == other.y
            self.x.partial_cmp(&other.x)
        }
    }
}

#[derive(Clone, Copy)]
struct Edge(Pos, Pos);

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (self.0 == other.0 && self.1 == other.1)
            || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Edge {}

impl std::hash::Hash for Edge {
    fn hash<H>(&self, state: &mut H)
            where H: std::hash::Hasher
    {
        if self.0 < self.1 {
            self.1.hash(state);
            self.0.hash(state);
        } else {
            self.0.hash(state);
            self.1.hash(state);
        }
    }
}

impl<A: Into<Pos>, B: Into<Pos>> From<(A, B)> for Edge {
    fn from((a, b): (A, B)) -> Edge {
        Edge(a.into(), b.into())
    }
}

struct Graph {
    start: Pos,
    end: Pos,
    edges: HashSet<Edge>,
}

fn main() {
    let graph = read_graph();
    for &Edge(a, b) in &graph.edges {
        println!("{:?} <-> {:?}", a, b);
    }
    println!("{}", graph.edges.contains(&((8, 3), (8, 2)).into()));
}

fn read_graph() -> Graph {
    let mut decoder = image::bmp::BMPDecoder::new(File::open("laberinto-1.bmp").unwrap());
    let (width, height) = decoder.dimensions().unwrap();
    let frames = decoder.into_frames().unwrap().next().unwrap();
    let buf = frames.into_buffer();
    let mut start = None;
    let mut end = None;
    let mut edges: HashSet<Edge> = HashSet::new();
    for (x, y, pixel) in buf.enumerate_pixels() {
        if pixel.data[0] == 255 {
            if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
                if buf.get_pixel(x-1, y).data[0] == 255 {
                    edges.insert(((x, y), (x-1, y)).into());
                }
                if buf.get_pixel(x+1, y).data[0] == 255 {
                    edges.insert(((x, y), (x+1, y)).into());
                }
                if buf.get_pixel(x, y-1).data[0] == 255 {
                    edges.insert(((x, y-1), (x, y)).into());
                }
                if buf.get_pixel(x, y+1).data[0] == 255 {
                    edges.insert(((x, y+1), (x, y)).into());
                }
            } else if y == 0 {
                start = Some((x, y).into());
            } else if y == height-1 {
                end = Some((x, y).into());
            }
        }
    }
    Graph {
        start: start.unwrap(),
        end: end.unwrap(),
        edges,
    }
}
