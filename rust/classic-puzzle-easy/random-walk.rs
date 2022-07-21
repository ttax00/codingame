use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let a = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let b = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = parse_input!(input_line, i32);

    let mut v = vec![0];
    let mut x = 0;
    let mut y = 0;
    let mut steps = 1;

    match d(steps, &mut v, (a,b,m)) % 4{
        0 => y += 1,
        1 => y -= 1,
        2 => x -= 1,
        3 => x += 1,
        _ => {}
    }

    while x != 0 || y != 0 {
        steps += 1;
        match d(steps, &mut v, (a,b,m)) % 4{
            0 => y += 1,
            1 => y -= 1,
            2 => x -= 1,
            3 => x += 1,
            _ => {}
        }
    }
    println!("{}", steps);

    fn d(n: i32, v: &mut Vec<i32>, (a, b, m): (i32, i32 ,i32)) -> i32{
        if v.get(n as usize).is_some() { v[n as usize] }
            else {
                let x = (a * d(n - 1, v, (a, b, m)) + b) % m;
                v.push(x);
                x
            }
    }
}
