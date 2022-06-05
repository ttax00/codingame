use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let r = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let v = parse_input!(input_line, i32);

    let mut time = vec![0; r];
    for i in 0..v as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let c = parse_input!(inputs[0], u32);
        let n = parse_input!(inputs[1], u32);

        let current_min = time.iter().min().unwrap();
        let min_index = time.iter().position(|i| i == current_min).unwrap();
        time[min_index] +=  combination_time(&c, &n);
        eprintln!("combination time {} - {} / {}", combination_time(&c, &n), c, n);

    }

    eprintln!("{:?}", time);
    println!("{}", time.iter().max().unwrap());

    fn combination_time(c: &u32, n: &u32) -> i32 {
        return i32::pow(10, *n) * i32::pow(5, c - n);
    }
}
