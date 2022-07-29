use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut houses = Vec::<(i64, i64)>::new();
    for _ in 0..input_line.trim().parse::<usize>().unwrap() {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = inputs[0].parse::<i64>().unwrap();
        let y = inputs[1].trim().parse::<i64>().unwrap();

        houses.push((x, y));
    }

    let x_spread = houses.iter().fold(0, |acc, i| acc.max(i.0)) - houses.iter().fold(i64::MAX, |acc, i| acc.min(i.0));
    houses.sort_by(|a, b| a.1.cmp(&b.1));
    let median = houses[houses.len()/2].1;
    let sum_cable = houses.iter().fold(0, |acc, i| acc + (i.1 - median).abs());

    println!("{}", sum_cable + x_spread);
}
