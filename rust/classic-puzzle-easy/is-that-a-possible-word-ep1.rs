use std::{io, collections::HashMap};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut buf = String::new();
    let _input = io::stdin().read_line(&mut buf).unwrap();
    let _states =io::stdin().read_line(&mut buf).unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let number_of_transitions = parse_input!(buf, i32);
    let mut transitions = HashMap::<String, String>::new();
    for i in 0..number_of_transitions as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let transition = input_line.trim_matches('\n').to_string();
        // save transition as {prior state}-{char} -> {new state}
        transitions.insert(format!("{}-{}", &transition[0..1], &transition[2..3]), transition[4..5].to_string());
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let start_state = input_line.trim_matches('\n').to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let end_states = input_line.trim_matches('\n').to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_words = parse_input!(input_line, i32);
    let mut current_state = &start_state;
    let unknown_state = String::from("UNKNOWN");
    
    for _i in 0..number_of_words as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let word = input_line.trim_matches('\n').to_string();
        for char in word.split("").filter(|s| !s.is_empty()) {
            let key = format!("{}-{}", current_state, char);
            current_state = match transitions.get(&key) {
                Some(x) => { x },
                None => { &unknown_state },
            }
        }
        if end_states.split_whitespace().collect::<Vec<&str>>().contains(&current_state.as_str()) {
            println!("true");
        } else {
            println!("false");
        }
        current_state = &start_state;
    }
}
