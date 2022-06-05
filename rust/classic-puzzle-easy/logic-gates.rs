use std::{io::stdin, collections::HashMap};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let std = stdin();
    let mut buf = String::new();
    std.read_line(&mut buf).unwrap();
    let n = parse_input!(buf, i32);
    let mut buf = String::new();
    std.read_line(&mut buf).unwrap();
    let m = parse_input!(buf, i32);

    let mut signals = HashMap::<String, Vec<String>>::new();
    for _i in 0..n as usize {
        let mut input_line = String::new();
        std.read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let input_name = inputs[0].trim().to_string();
        let input_signal = inputs[1].trim().to_string();

        signals.insert(input_name.to_string(), 
            input_signal.split("")
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect());
    }

    for _i in 0..m as usize {
        let mut input_line = String::new();
        std.read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let output_name = inputs[0].trim().to_string();
        let logic = inputs[1].trim().to_string();
        let input_name_1 = inputs[2].trim().to_string();
        let input_name_2 = inputs[3].trim().to_string();

        let signal_1 = signals.get(&input_name_1).unwrap();
        let signal_2 = signals.get(&input_name_2).unwrap();

        let mut answer = String::with_capacity(signal_1.len());
        for i in 0..signal_1.len() {
            let on = |is_on: bool| -> String { if is_on { "-".to_string() } else { "_".to_string() }};
            let and = |a: &String, b: &String| -> String { on(a == &"-" && a == b) };
            let or = |a: &String, b: &String| -> String { on(a == &"-" || b == &"-") };
            let xor = |a: &String, b: &String| -> String { on((a == &"-" && b == &"_") || (a == &"_" && b == &"-"))};
            let nand = |a: &String, b: &String| -> String { on(!(a == &"-" && a == b)) };
            let nor = |a: &String, b: &String| -> String { on(!(a == &"-" || b == &"-")) };
            let nxor = |a: &String, b: &String| -> String { on(!((a == &"-" && b == &"_") || (a == &"_" && b == &"-")))};
            match logic.as_str() {
                "AND" => answer.push_str(&and(&signal_1[i], &signal_2[i])),
                "OR" => answer.push_str(&or(&signal_1[i], &signal_2[i])),
                "XOR" => answer.push_str(&xor(&signal_1[i], &signal_2[i])),
                "NAND" => answer.push_str(&nand(&signal_1[i], &signal_2[i])),
                "NOR" => answer.push_str(&nor(&signal_1[i], &signal_2[i])),
                "NXOR" => answer.push_str(&nxor(&signal_1[i], &signal_2[i])),
                _ => {}
            }
        }

        println!("{} {}", output_name, answer)
    }

}
