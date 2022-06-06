use std::{io, collections::{HashMap, VecDeque}};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut resistors = HashMap::<String, f64>::new();
    for _i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let name = inputs[0].trim().to_string();
        let r = parse_input!(inputs[1], f64);
        resistors.insert(name, r);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let circuit = input_line.trim_matches('\n').to_string();
    
    fn parse(input: &mut VecDeque<&str>, resistors: &HashMap<String, f64>) -> f64 {
        let get_resistor = |r: &str| -> f64 {
            *resistors.get(r).unwrap()
        };
        //sample and remove the outer paren
        let peek = input.pop_front();
        input.pop_back();

        // figure out calculation for circuit if no circut, return the first resistor's value in queue
        let calculation = match peek {
            Some("(")  => |a: Vec<f64>| -> f64 {
                a.iter().sum()
            },
            Some("[") => |a: Vec<f64>| -> f64 {
                let sum = a.iter().map(|r| 1.0/ r ).sum::<f64>();
                1.0 / sum
            },
            Some(c) => return get_resistor(c),
            None => unreachable!(),
        };
        let mut v = Vec::<f64>::new();
        while input.len() > 0 {
            v.push(parse(&mut parse_even(input), resistors));
        }

        return calculation(v)

    }

    fn parse_even<'a>(input: &'a mut VecDeque<&str>) -> VecDeque<&'a str> {
        let peek = input.pop_front();
        let mut even = 0;
        let mut contents = VecDeque::<&'a str>::new();

        contents.push_back(peek.unwrap());
        match peek {
            Some("(") => even -=1,
            Some("[") => even -=1,
            Some(c) => {
                contents.push_back(c);
                return contents;
            }   
            None => unreachable!(),
        }

        while even < 0 {
            let next = input.pop_front();

            match next {
                Some("(") => even -=1,
                Some("[") => even -=1,
                Some(")") => even += 1,
                Some("]") => even += 1,
                Some(_) => {}
                None => return contents,
            }
            contents.push_back(next.unwrap());
        }
        contents
    }
    println!("{:.1}", parse(&mut circuit.split(' ').collect::<VecDeque<_>>(), &resistors));
}