use std::collections::BinaryHeap;
use std::collections::HashSet;
use async_cons_list::ConsList;
use std::sync::mpsc::channel;
use rayon::prelude::*;

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

const N: usize = 1000;

pub fn semi_parallel_breadth_first_search(graph: Graph) -> ConsList<Pos> {
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
        let (explored_s, explored_r) = channel();
        let (frontier_s, frontier_r) = channel();
        let (done_s, done_r) = channel();
        let n = match frontier.len() {
            n if n >= N => N,
            n => n,
        };
        let frontier_len = frontier.len();
        let mut frontier_ = Vec::with_capacity(n);
        for _ in 0..n {
            frontier_.push(frontier.pop().unwrap());
        }
        //println!("{}, {}", frontier_len, frontier_.len());
        frontier_.par_iter()
            //.take(n)
            .for_each_with((explored_s, frontier_s, done_s), |&mut (ref explored_s, ref frontier_s, ref done_s), node| {
                //let node = frontier.pop().unwrap();
                explored_s.send(node.pos);
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
                                done_s.send(child.path);
                                return
                                //return child.path
                            }
                            frontier_s.send(child);
                            //frontier.push(child);
                        }
                    }
                }
            });
        /*
        for _ in 0..n {
            frontier.pop();
        }
        */
        for pos in explored_r {
            explored.insert(pos);
        }
        for node in frontier_r {
            frontier.push(node);
        }
        for path in done_r {
            return path
        }
    }
}
