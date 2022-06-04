use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    // zero if no temperatures
    if n == 0 {
        println!("{}", 0);
        return;
    }

    let mut answer: i32 = 6000;
    for i in inputs.split_whitespace() {
        let t = parse_input!(i, i32);

        
        if t.abs() < answer.abs() {
            answer = t;
        } else if t == answer.abs() {
            answer = t;
        }
    }

    println!("{}", answer);
}
