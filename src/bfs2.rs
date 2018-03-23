use std::collections::BinaryHeap;
use std::collections::HashSet;
use cons_list::ConsList;
use std::sync::mpsc::channel;

use *;

fn distance(a: Pos, b: Pos) -> f32 {
    let ax = a.x as f32;
    let ay = a.y as f32;
    let bx = b.x as f32;
    let by = b.y as f32;
    ((ax-bx).powf(2.0) + (ay-by).powf(2.0)).sqrt()
}

#[derive(PartialEq, Clone, Debug)]
struct Node {
    pos: Pos,
    cost: f32,
    neighbours: [Option<Pos>; 4],
    path: ConsList<Pos>,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const N: usize = 8;

pub fn double_breadth_first_search(graph: Graph) -> ConsList<Pos> {
    let mut frontier: BinaryHeap<Node> = {
        let node = Node {
            pos: graph.start,
            cost: 0.0,
            neighbours: graph.nodes[&graph.start],
            path: ConsList::new(),
        };
        if node.pos == graph.end { return ConsList::new() }
        let mut frontier: BinaryHeap<Node> = BinaryHeap::new();
        frontier.push(node);
        frontier
    };
    let mut explored: HashSet<Pos> = HashSet::new();
    loop {
        //println!("{:?}\n\n", frontier);
        if frontier.is_empty() { panic!("Fallé. ¡Imposible!") }
        let mut explored_s = vec![];
        let mut frontier_s = vec![];
        let mut done_s = vec![];
        let n = match frontier.len() {
            n if n >= N => N,
            n => n,
        };
        let mut frontier_ = Vec::with_capacity(n);
        for _ in 0..n {
            frontier_.push(frontier.pop().unwrap());
        }
        frontier_.iter()
            //.take(n)
            .map(|node| {
                //let node = frontier.pop().unwrap();
                //println!("{:?}", node.pos);
                explored_s.push(node.pos);
                //explored.insert(node.pos);
                for neighbour in &node.neighbours {
                    if let &Some(neighbour) = neighbour {
                        let child = Node {
                            pos: neighbour,
                            cost: node.cost + distance(node.pos, neighbour),
                            neighbours: graph.nodes[&neighbour],
                            path: node.path.append(neighbour),
                        };
                        if !explored.contains(&neighbour) {
                            if neighbour == graph.end {
                                done_s.push(child.path);
                                return
                                //return child.path
                            }
                            frontier_s.push(child);
                            //frontier.push(child);
                        }
                    }
                }
            })
            .collect::<Vec<_>>();
        //println!("done");
        /*
        for _ in 0..n {
            frontier.pop();
            //print!(".");
        }
        */
        //println!("");
        //println!("Explored: {:?}", explored_s);
        //print!("Explored: ");
        for pos in explored_s {
            explored.insert(pos);
            //print!(".");
        }
        //println!("");
        //print!("Frontier: ");
        for node in frontier_s {
            frontier.push(node);
            //print!(".");
        }
        //println!("");
        for path in done_s {
            //println!("finished!");
            return path
        }
    }
}
