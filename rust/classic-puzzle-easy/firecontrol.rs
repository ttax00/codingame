use std::io;

fn main() {

    let mut map = Vec::<Vec<String>>::with_capacity(6);
    let mut fire = false;
    for _i in 0..6 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let text = input_line.trim_matches('\n').to_string();
        if text.contains("*") { fire = true };
        let row = text.split("").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        eprintln!("{:?}", row);
        map.push(row);
    }


    let mut tree_chopped = 0;
    for h in 0..map.len() {
        for w in 0..map[h].len() {
            match map[h][w].as_str() {
                "*" => {
                    for a in (h-2)..(h+3) {
                        for b in (w-2)..(w+3) {
                            match map.get_mut(a) {
                                Some(m) => match m.get_mut(b) {
                                    Some(c) => {
                                        match c.as_str() {
                                            "#" => {
                                                tree_chopped += 1;
                                                *c = "B".to_string(); 
                                            },
                                            _ => {}
                                        }
                                    },
                                    None => {},
                                },
                                None => {},
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    let mut success = true;
    for h in 0..map.len() {
        for w in 0..map[h].len() {
            match map[h][w].as_str() {
                "*" => {
                    for a in (h-2)..(h+3) {
                        if a == h {continue;};
                        for b in (w-2)..(h+3) {
                            if b == w {continue;};
                            match map.get_mut(a) {
                                Some(m) => match m.get_mut(b) {
                                    Some(c) => {
                                        eprintln!("c: {}", c);
                                        match c.as_str() {
                                            "*" => {
                                                success = false;
                                            },
                                            _ => {}
                                        }
                                    },
                                    None => {},
                                },
                                None => {},
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }


    if !fire {
        println!("RELAX");
        return;
    }

    if !success || tree_chopped == 0{
        println!("JUST RUN");
        return;
    }
    println!("{}", tree_chopped);
}
