use std::io;

fn read_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().into()
}

fn main() {
    let _ = read_line();
    let mut v_cards = read_line().split_whitespace()
                            .map(|i| i.parse::<i32>().expect("")).collect::<Vec<_>>();
    eprintln!("{:?}", v_cards);
    let mut total = 0;

    // combined the lowest health cards are the cheapest way
    while v_cards.len() > 1 {
        v_cards.sort();
        let c1 = v_cards.remove(0);
        let c2 = v_cards.remove(0);
        total += c1 + c2;
        v_cards.push(c1 + c2);
    } 

    println!("{}", total);
}
