use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let recipie = input_line.trim_matches('\n').to_string();
    
    let mut answer = String::new();
    for chunk in recipie.split(' ') {
        if chunk == "nl" { answer.push_str("\n"); continue; } //exception, no count
        let last_two = &chunk[chunk.len()-2..];
        let abbr: Option<&str> = match last_two {
            "sp" => Some(" "),
            "bS" => Some("\\"),
            "sQ" => Some("'"),
            _ => None
        };

        if let Some(abbr) = abbr {
            let count = chunk[..chunk.len()-2].parse::<usize>().unwrap();
            for _ in 0..count {
                answer.push_str(abbr);
            }
        } else {
            let count = chunk[..chunk.len()-1].parse::<usize>().unwrap();
            let char = &chunk[chunk.len()-1..];
            for _ in 0..count {
                answer.push_str(char);
            }
        }

    }

    println!("{}", answer);
}
