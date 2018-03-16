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

#[derive(Clone, Copy, Debug)]
struct Edge(Pos, Pos);

impl Edge {
    fn connects_to(self, pos: Pos) -> bool {
        self.0 == pos || self.1 == pos
    }
}

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

#[derive(Clone)]
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
    println!("{}", graph.edges.len());
    let new_graph = chain_all_edges(&graph);
    //println!("{}", graph.edges.contains(&((3, 1), (4, 1)).into()));
    println!("{}", new_graph.edges.len());
    for &Edge(a, b) in &new_graph.edges {
        println!("{:?} <-> {:?}", a, b);
    }
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

fn chain_all_edges(graph: &Graph) -> Graph {
    let mut new_graph = graph.clone();
    /*
    for &e in &graph.edges {
        for &f in &graph.edges {
            //println!("{}", new_graph.edges.len());
            chain_separate_edges(&mut new_graph, e, f);
        }
    }
    */
    for &e in &graph.edges {
        chain_in_node(&mut new_graph, e.0);
        chain_in_node(&mut new_graph, e.1);
        //chain_separate_edges(&mut new_graph, e, f);
    }
    new_graph
}

fn chain_edges(graph: &Graph) -> Graph {
    let mut new_graph = graph.clone();
    for &Edge(a, b) in &graph.edges {
        chain_adjacent_edges(&mut new_graph, a);
        chain_adjacent_edges(&mut new_graph, b);
    }
    new_graph
}

fn chain_adjacent_edges(graph: &mut Graph, init: Pos) {
    let north = init.y > 0 && graph.edges.contains(&(init, (init.x, init.y-1)).into());
    let south = graph.edges.contains(&(init, (init.x, init.y+1)).into());
    let east = graph.edges.contains(&(init, (init.x+1, init.y)).into());
    let west = init.x > 0 && graph.edges.contains(&(init, (init.x-1, init.y)).into());
    match (north, south, east, west) {
        (true, true, false, false) => {
            graph.edges.remove(&(init, (init.x, init.y-1)).into());
            graph.edges.remove(&(init, (init.x, init.y+1)).into());
            graph.edges.insert(((init.x, init.y-1), (init.x, init.y+1)).into());
            //chain_separate_edges(graph, (init.x, init.y-1).into(), (init.x, init.y+1).into());
        }
        (false, false, true, true) => {
            graph.edges.remove(&(init, (init.x+1, init.y)).into());
            graph.edges.remove(&(init, (init.x-1, init.y)).into());
            graph.edges.insert(((init.x-1, init.y), (init.x+1, init.y)).into());
        }
        _ => (),
    }
}

fn chain_in_node(graph: &mut Graph, pos: Pos) {
    let edges = graph.edges.iter().cloned().filter(|e| e.connects_to(pos)).collect::<Vec<_>>();
    if edges.len() == 2 {
        chain_separate_edges(graph, edges[0], edges[1]);
    }
}

fn chain_separate_edges(graph: &mut Graph, a: Edge, b: Edge) {
    if a == b || !graph.edges.contains(&a) || !graph.edges.contains(&b) {
        return
    }
    let (left, mid, right) = {
        if a.0 == b.0 {
            (a.1, a.0, b.1)
        } else if a.0 == b.1 {
            (a.1, a.0, b.0)
        } else if a.1 == b.0 {
            (a.0, a.1, b.1)
        } else if a.1 == b.1 {
            (a.0, a.1, b.0)
        } else {
            return
            //panic!("Las aristas no son adyacentes.");
        }
    };
    if graph.edges.iter().filter(|e| e.connects_to(mid)).count() == 2 {
        graph.edges.remove(&a);
        graph.edges.remove(&b);
        graph.edges.insert((left, right).into());
    }
}
