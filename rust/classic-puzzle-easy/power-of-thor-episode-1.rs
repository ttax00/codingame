use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    let mut current_x = initial_tx;
    let mut current_y = initial_ty;

    struct Vector {
        x: i32,
        y: i32,
    }
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.

        let vec = Vector {
            x: light_x - current_x,
            y: light_y - current_y
        };

        let mut answer = String::from("");

        if vec.y > 0 {
            answer += "S";
            current_y = current_y + 1;
        } else if vec.y < 0 {
            answer += "N";
            current_y = current_y - 1;
        } else if vec.y == 0 {
            answer += ""
        }

        if vec.x > 0 {
            answer += "E";
            current_x = current_x + 1;
        } else if vec.x < 0 {
            answer += "W";
            current_x = current_x - 1;
        } else if vec.x == 0 {
            answer += ""
        }

        // A single line providing the move to be made: N NE E SE S SW W or NW
        println!("{}", answer);
    }
}
