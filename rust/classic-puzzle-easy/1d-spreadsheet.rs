use std::{io, collections::HashMap};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Cell {
    operation: String,
    arg1: String,
    arg2: String,
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut values: Vec<Option<i32>> = vec![None ; n as usize];
    let mut cells = HashMap::<usize, Cell>::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let operation = inputs[0].trim().to_string();
        let arg1 = inputs[1].trim().to_string();
        let arg2 = inputs[2].trim().to_string();
        cells.insert(i, Cell {
            operation,
            arg1,
            arg2,
        });
    }

    fn cal(index: usize, cells: &HashMap<usize, Cell>, values: &mut Vec<Option<i32>>) -> i32{
        values[index].unwrap_or_else(|| {
            let mut parse = |cell: &Cell| -> i32 {
                let mut parse_arg = |arg: &String| -> i32 {
                    let test = arg.parse::<i32>();
                    match test {
                        Ok(x) => x,
                        Err(_) => {
                            let index = arg.replace("$", "").parse::<usize>().unwrap();
                            cal(index, cells, values)
                        },
                    }
                };
    
                match cell.operation.as_str() {
                    "VALUE" => parse_arg(&cell.arg1),
                    "ADD" => parse_arg(&cell.arg1) + parse_arg(&cell.arg2),
                    "SUB" => parse_arg(&cell.arg1) - parse_arg(&cell.arg2),
                    "MULT" => parse_arg(&cell.arg1) * parse_arg(&cell.arg2),
                    _ => { panic!("Invalid operation") }
                }
            };
            let val = parse(cells.get(&index).unwrap());
            values[index] = Some(val);
            val
        })
    }

    for i in 0..n as usize {
        println!("{}", cal(i, &cells, &mut values));
    }
}
