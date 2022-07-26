use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();


    let mut diff = 0; 
    let mut tip = 0;
    for i in inputs.split_whitespace() {
        let data_point = parse_input!(i, i32);
        if tip < data_point { tip = data_point; }
        if tip > data_point && tip - data_point > diff { diff = tip - data_point; }
    }

    println!("{}", -diff);
}
