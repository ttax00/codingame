use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut v_horse = Vec::<i32>::new();
    
    for _i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, i32);
        v_horse.push(pi);
    }
    v_horse.sort();

    let mut prev = v_horse[0];
    let mut v_diff = Vec::<i32>::new();
    for i in 1..v_horse.len() {
        v_diff.push(v_horse[i] - prev);
        prev = v_horse[i];
    }

    v_diff.sort();
    println!("{:?}", v_diff[0]);
}
