use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        let queue = line.split("").filter(|s| !s.is_empty())
                    .map(|c| {
                        let position = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", 
                                        "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
                        let p = position.iter().position(|x| x == &c);  
                        match p {
                            Some(i) => return i,
                            None => unreachable!(),
                        }
                    } )
                    .collect::<Vec<usize>>();


        let mut stacks = Vec::<Vec<usize>>::new();
        for container in queue {

            if stacks.len() == 0 {
                stacks.push(vec![container]);
                continue;
            } 
            
            for i in 0..stacks.len() {
                let stack = &mut stacks[i];
                let top = stack[stack.len() - 1];

                if top == container {
                    stack.push(container);
                    break;
                } else if top > container {
                    stack.push(container);
                    break;
                } else if top < container {
                    if i == stacks.len() -1 {
                        stacks.push(vec![container]);
                    }
                    continue;
                }

            }

        }
        println!("{}", stacks.len());
    }

}
