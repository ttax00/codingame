use std::io::stdin;
use std::collections::HashSet;

fn main() {
    let mut input_line = String::new(); stdin().read_line(&mut input_line).unwrap();
    let n = input_line.trim().parse::<usize>().unwrap();

    let mut items = HashSet::<String>::new();
    for _ in 0..n {
        let mut input_line = String::new(); stdin().read_line(&mut input_line).unwrap();
        let tel = input_line.trim().to_string();
        for i in 0..tel.len() {
            items.insert(tel[0..i+1].to_string());
        }
    }
    println!("{}", items.len());
}
