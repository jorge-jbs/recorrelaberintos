use std::collections::HashSet;
use async_cons_list::ConsList;
use std::sync::mpsc::channel;
use rayon::prelude::*;

use *;

enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(PartialEq, Clone, Debug)]
struct Node {
    pos: Pos,
    neighbours: [Option<Pos>; 4],
    path: ConsList<Pos>,
}

unsafe impl Sync for Node {}

pub fn parallel_breadth_first_search(graph: Graph) -> ConsList<Pos> {
    let mut frontier = vec![Node {
            pos: graph.start,
            neighbours: graph.nodes[&graph.start],
            path: ConsList::new(),
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
        frontier = match process_level(&graph, &mut explored, frontier) {
            Either::Left(new_frontier) => new_frontier,
            Either::Right(path) => return path,
        };
        i += 1;
    }
    ConsList::new().append((0, 0).into())
}

fn process_level(graph: &Graph, explored: &mut HashSet<Pos>, frontier: Vec<Node>) -> Either<Vec<Node>, ConsList<Pos>> {
    let (sender, receiver) = channel();
    let (sender_finished, receiver_finished) = channel();
    for n in &frontier {
        explored.insert(n.pos);
    }
    frontier.into_par_iter()
        .for_each_with((sender, sender_finished), |&mut (ref sender, ref sender_finished), node| {
            //sender.send(node);
            for neighbour in &node.neighbours {
                if let &Some(neighbour) = neighbour {
                    let child = Node {
                        pos: neighbour,
                        neighbours: graph.nodes[&neighbour],
                        path: node.path.append(neighbour),
                    };
                    if !explored.contains(&neighbour) {
                        if neighbour == graph.end {
                            sender_finished.send(child.path.clone());
                            //return child.path
                        }
                        sender.send(child);
                        //frontier.push(child);
                    }
                }
            }
        });
    match receiver_finished.iter().next() {
        Some(path) => {
            println!("ciao!");
            Either::Right(path)
        }
        None => Either::Left(receiver.iter().collect()),
    }
}
