use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    // parse compound
    let mut graph = String::new();
    for _i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.to_string();
        graph.push_str(&line);
    };
    
    // create graph
    let v_graph: Vec<Vec<&str>> = graph.lines().map(|l| l.split("").collect()).collect();
    for line in &v_graph {
        eprintln!("{:?}", line);
    }

    // solve graph
    if solve_graph(&v_graph) {
        println!("VALID");
    } else {
        println!("INVALID");
    }
}

fn solve_graph(v_graph: &Vec<Vec<&str>>) -> bool {

    let mut res = true;
    for row in 0..v_graph.len() {
        for col in 0..v_graph[row].len() {
            
            if v_graph[row][col] == "C" {
                eprintln!("checking {} / {}", row, col);
                if !check_carbon(row, col, v_graph) {
                    res = false;
                    eprintln!("check failed {} / {}", row, col);

                }
            }
        }
    }
    return res
}

fn parse_num_at(row: usize, col: usize, v_graph: &Vec<Vec<&str>>) -> i32 {
    if row >= v_graph.len() { 0 }
    else if  col >= v_graph[row].len() { 0 }
    else { 
        match v_graph[row][col].parse::<i32>() {
            Ok(x) => { x },
            Err(_e) => { 0 }
        }
    }
}

fn check_carbon(row: usize, col: usize, v_graph: &Vec<Vec<&str>>) -> bool {
    let total_bonds = parse_num_at(row, col - 2, v_graph)
                    + parse_num_at(row, col + 4, v_graph)
                    + parse_num_at(row - 1, col + 1, v_graph)
                    + parse_num_at(row + 1, col + 1, v_graph);
    let hydrogen = parse_num_at(row, col + 2, v_graph);
    eprintln!("bonds: {}, hydrogen: {}", total_bonds, hydrogen);
    if total_bonds + hydrogen != 4 { false } 
    else { true }
}
