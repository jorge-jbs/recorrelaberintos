use std::collections::HashSet;
use cons_list::ConsList;
use std::sync::mpsc::channel;
use rayon::prelude::*;

use *;

#[derive(PartialEq, Clone, Debug)]
struct Node {
    pos: Pos,
    neighbours: [Option<Pos>; 4],
    //path: ConsList<Pos>,
}

unsafe impl Sync for Node {}

pub fn parallel_breadth_first_search(graph: Graph) -> ConsList<Pos> {
    let mut frontier = vec![Node {
            pos: graph.start,
            neighbours: graph.nodes[&graph.start],
            //path: ConsList::new(),
    }];
    let mut explored: HashSet<Pos> = HashSet::new();
    let mut i = 0;
    while !frontier.is_empty() && i < 100 {
        //println!("{:?}", frontier);
        print!("{} ", frontier.len());
        if true || i % 10 == 0 {
            use std::io::Write;
            ::std::io::Stdout::flush(&mut ::std::io::stdout());
        }
        frontier = match process_level(&graph, &explored, frontier) {
            Some(new_frontier) => new_frontier,
            None => break,
        };
        i += 1;
    }
    ConsList::new()
}

fn process_level(graph: &Graph, explored: &HashSet<Pos>, frontier: Vec<Node>) -> Option<Vec<Node>> {
    let (sender, receiver) = channel();
    let (sender_finished, receiver_finished) = channel();
    frontier.into_par_iter()
        .for_each_with((sender, sender_finished), |&mut (ref sender, ref sender_finished), node| {
            //sender.send(node);
            for neighbour in &node.neighbours {
                if let &Some(neighbour) = neighbour {
                    let child = Node {
                        pos: neighbour,
                        neighbours: graph.nodes[&neighbour],
                        //path: node.path.append(neighbour),
                    };
                    if !explored.contains(&neighbour) {
                        if neighbour == graph.end {
                            sender_finished.send(true);
                            //return child.path
                        }
                        sender.send(child);
                        //frontier.push(child);
                    }
                }
            }
        });
    if receiver_finished.iter().next() == Some(true) {
        println!("ciao!");
        None
    } else {
        Some(receiver.iter().collect())
    }
}
