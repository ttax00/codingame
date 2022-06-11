use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let r_1 = parse_input!(input_line, i64);
    let mut meet = false;
    for r_2 in 1.max(r_1 -40)..r_1 {
        if  r_1 == r_2 + r_2.to_string().split("")
                            .filter_map(|s| s.parse::<i64>().ok())
                            .sum::<i64>(){
            meet = true;
        }
    }
    match meet {
        true => println!("YES"),
        false => println!("NO"),
    }
}
