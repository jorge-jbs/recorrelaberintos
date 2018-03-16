extern crate image;

use std::fs::File;
use std::rc::Rc;
use std::collections::HashSet;
use std::collections::HashMap;
use image::ImageDecoder;

#[derive(Clone, Copy, Debug)]
enum NodeType {
    Start,  // node with one sibling at the top of the image
    End,  // node with one sibling at the bottom of the image
    Joint,  // node with two siblings
    Corner,  // node with more than two siblings
}

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
    let nodes = read_nodes();
    //nodes.iter().inspect(|&&(pos, t)| println!("{:?} - {:?}", pos, t)).collect::<Vec<_>>();
    //println!("");
    let graph = generate_graph(nodes);
    graph.edges.iter().inspect(|&&Edge(a, b)| println!("{:?} <-> {:?}", a, b)).collect::<Vec<_>>();
    println!("{}", graph.edges.contains(&((8, 3), (8, 2)).into()));
}

fn read_nodes() -> Vec<(Pos, NodeType)> {
    let mut decoder = image::bmp::BMPDecoder::new(File::open("laberinto-1.bmp").unwrap());
    let (width, height) = decoder.dimensions().unwrap();
    let frames = decoder.into_frames().unwrap().next().unwrap();
    let buf = frames.into_buffer();
    let mut nodes: Vec<(Pos, NodeType)> = Vec::new();
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
                nodes.push(((x, y).into(), if n <= 2 { NodeType::Joint } else { NodeType::Corner }));
            } else {
                if y == 0 {
                    nodes.push(((x, y).into(), NodeType::Start));
                } else if y == height-1 {
                    nodes.push(((x, y).into(), NodeType::End));
                }
            }
        }
    }
    nodes
}

fn generate_graph(nodes: Vec<(Pos, NodeType)>) -> Graph {
    let mut hor_edges: HashSet<Edge> = HashSet::new();
    let mut last: Pos = nodes[0].0;
    for &(pos, t) in nodes.iter().skip(1).rev().skip(1).rev() {
        if last.x+1 == pos.x && last.y == pos.y {
            hor_edges.insert((last, pos).into());
        }
        last = pos;
    }

    let mut ver_edges: HashSet<Edge> = HashSet::new();
    for &(pos, t) in nodes.iter() {
        if nodes.iter().find(|&&(suc, _)| pos.x == suc.x && pos.y+1 == suc.y).is_some() {
            ver_edges.insert((pos, (pos.x, pos.y+1)).into());
        }
    }
    Graph {
        start: nodes[0].0,
        end: nodes.last().unwrap().0,
        edges: hor_edges.union(&ver_edges).cloned().collect(),
    }
}
