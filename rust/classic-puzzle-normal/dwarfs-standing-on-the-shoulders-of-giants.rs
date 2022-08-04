use std::io;
use std::collections::{HashMap, HashSet};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
#[derive(Debug)]
struct Node {
    influences: HashSet<u32>,
    parents: HashSet<u32>,
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of relationships of influence
    let mut graph: HashMap<u32, Node> = HashMap::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], u32); // a relationship of influence between two people (x influences y)
        let y = parse_input!(inputs[1], u32);
        if let Some(node) = graph.get_mut(&x) {
            node.influences.insert(y);
        } else {
            let mut influences = HashSet::new();
            influences.insert(y);
            graph.insert(x, Node{
                influences: influences,
                parents: HashSet::new(),
            });
        }
        if let Some(node) = graph.get_mut(&y) {
            node.parents.insert(x);
        } else {
            let mut parents = HashSet::new();
            parents.insert(x);
            graph.insert(y, Node{
                influences: HashSet::new(),
                parents: parents,
            });
        }
    }

    fn find_depth(key: &u32, graph: &HashMap<u32, Node>) -> u32{
        let mut depth = 0;
        if let Some((_, node)) = graph.get_key_value(key) {
            depth += 1;
            if node.influences.is_empty() {
                return depth;
            } else {
                depth += node.influences.iter().fold(0, |acc, n| {
                    let new_depth = find_depth(n, graph);
                    if new_depth > acc { new_depth }
                    else { acc }
                });
            }
        }
        depth
    }

    let origins = graph.iter().filter(|(_, node)| node.parents.is_empty()).collect::<HashMap<_,_>>();

    let answer = origins.iter().fold(0, |acc, (n, _)| {
        let new_depth = find_depth(n, &graph);
        if new_depth > acc { new_depth }
        else { acc }
    });
    
    println!("{}", answer);
}
