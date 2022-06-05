use std::{io, collections::{VecDeque, HashMap}};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let n = parse_input!(inputs[0], usize); // the total number of nodes in the level, including the gateways
    let l = parse_input!(inputs[1], i32); // the number of links
    let e = parse_input!(inputs[2], i32); // the number of exit gateways
   
    // create empty 2d link vector 
    let mut v_links = vec![vec![false; n]; n];

    // populate links
    for _i in 0..l {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line_string = input_line.trim_matches('\n').to_string();
        let v_line = line_string.split(' ').map(|c| c.parse::<usize>().unwrap())
                                            .collect::<Vec<usize>>();
        eprintln!("{}-{}", v_line[0], v_line[1]);

        //remember links go both way
        v_links[v_line[0]][v_line[1]] = true;
        v_links[v_line[1]][v_line[0]] = true;
    }
    

    // create empty exit check vector
    let mut v_exits = vec![false; n];

    for _i in 0..e {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line_string = input_line.trim_matches('\n').to_string();
        let exit_index = line_string.parse::<usize>().unwrap();
        
        eprintln!("exits: {}", exit_index);
    
        v_exits[exit_index] = true;
    }

    eprintln!("{:?}", v_links);
    eprintln!("{:?}", v_exits);


    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let si = parse_input!(input_line, usize); // The index of the node on which the Bobnet agent is positioned this turn

        let r = breath_path_to_nearest_exit(&v_links, &si, &v_exits);

        // Example: 0 1 are the indices of the nodes you wish to sever the link between
        println!("{} {}", r[0], r[1]);
    }

    fn is_exit(index: usize, v_exits: &Vec<bool>) -> bool {
        match v_exits.get(index) {
            Some(bool) => { *bool },
            None => { false },
        }
    }

    fn has_link(a: &usize, b: &usize, v_links: &Vec<Vec<bool>>) -> bool {
        match v_links.get(*a) {
            Some(v_b) => {
                match v_b.get(*b) {
                    Some(a) => { *a },
                    None => { false },
                }
            },
            None => { false },
        }
    }

    fn breath_path_to_nearest_exit(v_links: &Vec<Vec<bool>>, origin: &usize, v_exits: &Vec<bool>) -> Vec<usize> {
        let mut queue = VecDeque::<usize>::from(vec![*origin]);
        let mut visited = Vec::<usize>::new();
        let mut tree = HashMap::<usize, usize>::new();
        let mut exit: usize = 0;

        // bfs - build parent tree & find nearest exit
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            let neighbours = get_neighbour_v(v_links, &node);

            eprintln!("processing node: {}", node);
            eprintln!("{:?}", queue);

            for neighbour in neighbours.iter() {
                if visited.contains(&neighbour) {
                    continue;
                } else {
                    eprintln!("visited: {} from: {}", neighbour, node);
                    visited.push(*neighbour);
                    tree.insert(*neighbour, node);
                }
            }

            if is_exit(node, v_exits) {
                eprintln!("nearest exit: {}", node);
                exit = node;
                break;
            } else {
                queue.append(&mut VecDeque::from(neighbours));
            };
        }

        // solve parent tree
        let mut path = vec![exit];
        let mut current = &exit;
        eprintln!("{:?}", tree);
        while current != origin {
            eprintln!("current {}", current);
            current = tree.get(current).unwrap();
            path.push(*current);
        }
        path.reverse();

        return path;

    }

    fn get_neighbour_v(v_links: &Vec<Vec<bool>>, origin: &usize) -> Vec<usize> {
        let mut queue = Vec::<usize>::new();
        for i in 0..v_links.len() {
            if has_link(origin, &i, v_links) {
                queue.push(i)
            }
        }
        eprintln!("neighbours: {:?}", queue);
        return queue
    }
}
