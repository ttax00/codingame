use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut c: i32 = -1;
    let mut inputs = input_line.trim_matches('\n')
		.split(' ')
		.filter(|s| !s.is_empty())
		.map(|s| { c+=1;  (s.to_string(), c) })
		.collect::<Vec<(String, i32)>>();

    let mut exits = vec![0;inputs.len()];
    for _ in 0..h-2 {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string()
            .split('|').filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        eprintln!("line {:?}", line);
        for a in 0..line.len() {
            if line[a] == "--" {
                for input in inputs.iter_mut() {
                    if input.1 as usize == a {
                        input.1 += 1;
                    } else if input.1 as usize == a + 1 {
                        input.1 -= 1;
                    }
                }
                exits[a] += 1;
                exits[a + 1] -= 1;
            }
        }
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let exits = input_line.trim_matches('\n')
        .split(' ')
        .filter(|a| !a.is_empty())
        .map(|a| a.to_string())
        .collect::<Vec<String>>();
    eprintln!("{:?}", exits);
    for i in inputs {
        println!("{}{}", i.0, exits[i.1 as usize]);
    }
}
