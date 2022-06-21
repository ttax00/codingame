use std::{io, convert::TryInto};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut nums = vec![("0".to_string(), 0), ("T".to_string(), -1), ("1".to_string(),1)];
    let int_to_ternary = |i: &i32| -> &str {
        match i {
            0 => "0",
            1 => "1",
            -1 => "T",
            _ => panic!("should only get -1, 0, 1")
        }
    };
    loop {
        let (a,b) = nums.remove(0);
        for i in [-1,0,1].iter() {
 
            let value = i * 3_i32.pow(a.len().try_into().unwrap()) + b;
            let string = format!("{}{}", int_to_ternary(i), a) ;
            if i == &n {
                println!("{}", int_to_ternary(&n));
                return;
            } else if value == n {
                println!("{}", string);
                return;
            }
            nums.push((string, value))
        }

    }
}
