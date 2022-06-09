use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut r_1 = parse_input!(input_line, i64);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut r_2 = parse_input!(input_line, i64);

    while r_1 != r_2 {
        if r_1 < r_2 {
            r_1 += r_1.to_string().split("")
                    .filter(|i| !i.is_empty())
                    .map(|i| i.parse::<i64>().unwrap())
                    .sum::<i64>();
        } else {
            r_2 += r_2.to_string().split("")
            .filter(|i| !i.is_empty())
            .map(|i| i.parse::<i64>().unwrap())
            .sum::<i64>();
        }
    }
    println!("{}", r_1);
}
