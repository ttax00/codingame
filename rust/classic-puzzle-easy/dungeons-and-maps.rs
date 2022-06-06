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
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let start_row = parse_input!(inputs[0], i32);
    let start_col = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let n = parse_input!(input_line, i32);
    let mut min_len = i32::MAX;
    let mut map_id = 0;
    for i in 0..n as usize {
        let mut map = Vec::<Vec<String>>::new();
        // build map
        for j in 0..h as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let map_row = input_line.trim_matches('\n').to_string().split("")
                                    .skip_while(|s| s == &"") // remove "" from "ab".split("") == ["", "a", "b", ""]
                                    .map(|s| s.to_string()).collect::<Vec<String>>();
            map.push(map_row);
        }

        // solve map for len recusively
        fn solve(h: usize, r: usize, l: i32, map: &Vec<Vec<String>>) -> i32 {
            let mut l = l;
            if l > 100 { return i32::MAX; } // arbitrary, exit if looks to be infinite loop

            match map.get(h) {
                Some(row) => {
                    match row.get(r) {
                        Some(c) => {
                            l += 1;
                            match c.as_str() {
                                "." => i32::MAX,
                                "#" => i32::MAX,
                                "^" => solve(h-1, r, l, map),
                                "v" => solve(h+1, r, l, map),
                                ">" => solve(h, r + 1, l, map),
                                "<" => solve(h, r - 1, l, map),
                                "T" => l,
                                _ => i32::MAX,
                            }
                        },
                        None => i32::MAX,
                    }
                },
                None => i32::MAX,
            }
        }

        let len = solve(start_row as usize, start_col as usize, 1, &map);
        eprintln!("map: {} len: {}", i, len);

        if len < min_len {
            min_len =len;
            map_id = i;
        }

    }
    if min_len == i32::MAX {
        println!("TRAP");
    } else {
        println!("{}", map_id);
    }
}
