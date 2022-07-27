use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let _ = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let mut w0 = parse_input!(inputs[0], i32);
    let mut h0 = parse_input!(inputs[1], i32);
    let (mut w_min, mut h_min, mut w_max, mut h_max) = (0, 0, w, h);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        for dir in bomb_dir.split("") {
            match dir {
                "U" => { h_max = h0; h0 = h_min + (h0-h_min)/2; },
                "D" => { h_min = h0; h0 = h0 + (h_max-h0)/2 },
                "L" => { w_max = w0; w0 = w_min + (w0-w_min)/2; },
                "R" => { w_min = w0; w0 = w0 + (w_max-w0)/2 },
                _ => {/* do nothing*/}
            }
        }
        // eprintln!("{} - {} {} {} - {} {} {}", bomb_dir, w0, w_min, w_max, h0, h_min, h_max);

        println!("{} {}", w0, h0);
    }
}
