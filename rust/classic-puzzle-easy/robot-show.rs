use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let inputs = inputs.split_whitespace().collect::<Vec<&str>>();
    
    // Explaination: When bots collide, they turn around. 
    // But since both move at the same speed, its indifferent to if they had move through eachother
    // Thus the longest time is only taken up by how long does the furthest bot take to reach exit.
    // Ted-Ed riddle : https://www.youtube.com/watch?v=zoZVuqP1rQM
    let positions: Vec<i32> = inputs.iter().map(|p| p.parse::<i32>().expect("position is not a number")).collect();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    eprintln!("{:?}", positions);
    if l - min < max {
        println!("{}", max);
    } else {
        println!("{}", l-min)
    }  
}
