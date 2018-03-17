extern crate image;

use std::fs::File;
use std::collections::HashMap;
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

struct Graph {
    start: Pos,
    end: Pos,
    nodes: HashMap<Pos, [Option<Pos>; 4]>,
}

fn main() {
    let graph = read_graph();
    for (pos, edges) in &graph.nodes {
        println!("{:?} -> {:?}", pos, edges);
    }
    println!("{}", graph.nodes.len());
}

fn read_graph() -> Graph {
    let (buf, width, height) = {
        let mut decoder = image::bmp::BMPDecoder::new(File::open("laberinto-1.bmp").unwrap());
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

fn distance(a: Pos, b: Pos) -> f32 {
    ((a.x+b.x).pow(2) as f32 + (a.y+b.y).pow(2) as f32).sqrt()
}

#[derive(PartialEq, Clone, Debug)]
struct Node {
    pos: Pos,
    distance: f32,
    neighbours: [Option<Pos>; 4],
    path: Vec<Pos>,
}

fn breadth_first_search(graph: Graph) -> Vec<Pos> {
    let node = Node {
        pos: graph.start,
        distance: distance(graph.start, graph.end),
        neighbours: graph.nodes[&graph.start],
        path: Vec::new(),
    };
    if node.pos == graph.end { return Vec::new() }
    let mut frontier: Vec<Node> = vec![node];
    let mut explored: HashSet<Pos> = HashSet::new();
    loop {
        //println!("{:?}", frontier.iter().map(|x| x.pos).collect::<Vec<_>>());
        if frontier.is_empty() { panic!("Fallé. ¡Imposible!") }
        let nodo = frontier.pop().unwrap();
        explored.insert(nodo.pos);
        for neighbour in &nodo.neighbours {
            if let &Some(neighbour) = neighbour {
                let mut new_path = nodo.path.clone();
                new_path.push(neighbour);
                let child = Node {
                    pos: neighbour,
                    distance: distance(nodo.pos, neighbour),
                    neighbours: graph.nodes[&neighbour],
                    path: new_path,
                };
                if !explored.contains(&neighbour) && frontier.iter().find(|&x| x.clone() == child).is_none() {
                    if neighbour == graph.end {
                        return child.path
                    }
                    frontier.insert(0, child);
                }
            }
        }
    }
}
