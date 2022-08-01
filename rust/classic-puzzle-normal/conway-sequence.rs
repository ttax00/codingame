use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let r = input_line.trim().parse::<i32>().unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = input_line.trim().parse::<i32>().unwrap();

    let mut seq = vec![r];

    for _ in 0..l-1 {
        let mut new_seq: Vec<i32> = vec![];
        let mut count = 0;
        let mut current = seq[0];

        for num in seq.iter() {
            if current == *num {
                count += 1;
            } else {
                new_seq.extend(&[count, current]);
                current = *num;
                count = 1;
            }
        }
        new_seq.extend(&[count, current]);
        seq = new_seq;
    }

    println!("{}", seq.iter().map(|i| format!("{} ", i)).collect::<String>().trim());
}
