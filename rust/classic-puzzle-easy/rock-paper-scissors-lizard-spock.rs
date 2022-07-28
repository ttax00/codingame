use std::{io, collections::{VecDeque, HashMap}};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
#[derive(Debug)]
struct Player {
    id: i32,
    play: String,
    faced: Vec<i32>,
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut queue = VecDeque::<Player>::new();

    let mut rules = HashMap::<&str, bool>::new();
    rules.insert("C-P", true);
    rules.insert("P-R", true);
    rules.insert("R-L", true);
    rules.insert("L-S", true);
    rules.insert("S-C", true);
    rules.insert("C-L", true);
    rules.insert("L-P", true);
    rules.insert("P-S", true);
    rules.insert("S-R", true);
    rules.insert("R-C", true);

    rules.insert("P-C", false);
    rules.insert("R-P", false);
    rules.insert("L-R", false);
    rules.insert("S-L", false);
    rules.insert("C-S", false);
    rules.insert("L-C", false);
    rules.insert("P-L", false);
    rules.insert("S-P", false);
    rules.insert("R-S", false);
    rules.insert("C-R", false);

    for i in (0..n as usize).step_by(2){
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n1 = parse_input!(inputs[0], i32);
        let s1 = inputs[1].trim();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n2 = parse_input!(inputs[0], i32);
        let s2 = inputs[1].trim();
        let mut enq = |n: i32, s: String, faced: i32| {
            queue.push_back(Player  {
                id: n,
                play: s,
                faced: vec![faced],
            });
        };

        if s1 == s2 {
            // lower number win by default
            if n1 < n2 {
                enq(n1, s1.to_string(), n2);
            } else {
                enq(n2, s2.to_string(), n1);
            }
        } else {
            // determine winner
            if *rules.get(format!("{}-{}", s1, s2).as_str()).unwrap() {
                enq(n1, s1.to_string(), n2);
            } else {
                enq(n2, s2.to_string(), n1);
            }
        }

        // eprintln!("Set: {} - {} / {} - {}", n1, s1, n2, s2);
        // eprintln!("queue: {:?}", queue);
    }

    while queue.len() > 1 {
        let mut p1 = queue.pop_front().unwrap();
        let mut p2 = queue.pop_front().unwrap();
        let mut enq = |p: Player| {
            queue.push_back(p);
        };

        if p1.play == p2.play {
            // lower number win by default
            if p1.id < p2.id {
                p1.faced.push(p2.id);
                enq(p1);
            } else {
                p2.faced.push(p1.id);
                enq(p2);
            }
        } else {
            // determine winner
            if *rules.get(format!("{}-{}", p1.play, p2.play).as_str()).unwrap() {
                p1.faced.push(p2.id);
                enq(p1);
            } else {
                p2.faced.push(p1.id);
                enq(p2);
            }
        }
    }

    let winner = queue.pop_front().unwrap();
    let mut faced = String::from("");
    for f in winner.faced {
        faced.push_str(format!(" {}", f).as_str());
    }
    println!("{}", winner.id);
    println!("{}",faced.trim());
}
