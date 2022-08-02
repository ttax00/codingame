use std::io;
use chrono::NaiveTime;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let mut smallest = NaiveTime::from_hms(23, 59, 59);
    for i in 0..input_line.trim().parse::<usize>().unwrap() {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        
        let time = NaiveTime::parse_from_str(input_line.trim(), "%H:%M:%S").unwrap();
        if time < smallest {
            smallest = time;
        }
    }

    println!("{}", smallest.format("%H:%M:%S"));
}
