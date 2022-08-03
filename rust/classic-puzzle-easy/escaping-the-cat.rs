use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Implementing tactitc from following video:
 * https://www.youtube.com/watch?v=vF_-ob9vseM
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let cat_speed = parse_input!(input_line, f32);
    let radius = 500.0;
    let pi = std::f32::consts::PI;
    let circumference = 2.0 * radius * pi;
    let cat_speed = cat_speed as f32 / circumference;
    let mouse_speed = 10.0;
    let circumference = mouse_speed / cat_speed;
    let radius_mouse = circumference / 2.0 / pi;
    let ratio = (radius_mouse - 5.0) / radius;
    eprintln!("mouse {}", radius_mouse);
    let mut escaping = false;

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let mouse_x = parse_input!(inputs[0], f32);
        let mouse_y = parse_input!(inputs[1], f32);
        let cat_x = parse_input!(inputs[2], f32);
        let cat_y = parse_input!(inputs[3], f32);
        let point_x = (-cat_x as f32 * ratio).floor();
        let point_y = (-cat_y as f32 * ratio).floor();
        if escaping {
            println!("{} {} Escaping", (mouse_x * 2.0).ceil(), (mouse_y * 2.0).ceil());
        } else {
            let dist_to_escape = ((point_x - mouse_x).powf(2.0) + (point_y - mouse_y).powf(2.0)).sqrt();
            if ratio < 0.5 {
                if dist_to_escape < 10.0 {
                    escaping = true;
                }
            } else {
                if dist_to_escape < 80.0 {
                    escaping = true;
                }
            }

            eprintln!("dist {} ratio {}", dist_to_escape, ratio);
            println!("{} {} Circling", point_x, point_y);
        }
 
    }
}
