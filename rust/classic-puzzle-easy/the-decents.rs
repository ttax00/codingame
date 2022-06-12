use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    struct Mountain(i32, i32);
    loop {
        let mut mountains = (0..8).map(|i| {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            Mountain(i, parse_input!(input_line, i32))
        }).collect::<Vec<_>>();

        mountains.sort_by(|a, b| a.1.cmp(&b.1));
        mountains.reverse();
        println!("{}", mountains[0].0); // The index of the mountain to fire on.
    }
}
