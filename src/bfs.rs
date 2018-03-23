use std::collections::BinaryHeap;
use std::collections::HashSet;
use cons_list::ConsList;

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

pub fn breadth_first_search(graph: &Graph) -> ConsList<Pos> {
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
        if frontier.is_empty() { panic!("Fallé. ¡Imposible!") }
        let node = frontier.pop().unwrap();
        if !explored.insert(node.pos) {  // if he have already explored the node, skip it
            /** Instead of checking duplicates in `frontier` (finding whether a node is in the
             * frontier is expensive), we add nodes even if they are duplicated, and skip them we
             * have already explored them.
             *
             * Although in practice there doesn't seem to be any duplicated nodes :/
             */
            println!("eh!");
            continue
        }
        for neighbour in &node.neighbours {
            if let Some(neighbour) = *neighbour {
                let child = Node {
                    pos: neighbour,
                    cost: node.cost + distance(node.pos, neighbour),
                    neighbours: graph.nodes[&neighbour],
                    path: node.path.append(neighbour),
                };
                if !explored.contains(&neighbour) {
                    if neighbour == graph.end {
                        return child.path
                    }
                    frontier.push(child);
                }
            }
        }
    }
}
