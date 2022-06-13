use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], usize);
    let h = parse_input!(inputs[1], usize);
    let mut map = Vec::<_>::new();
    for _i in 0..h as usize {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        map.push(inputs.split_whitespace().map(|i| i.parse::<i32>().expect("")).collect::<Vec<i32>>());
    }

    for h in 0..h {
        for w in 0..w {
            // solve for optimal for each cell
            let mut a = h;
            let mut b = w;
            let mut c = map[h][w]; // value of true rec
            // cut and compare
            while a > 0 {
                a -= 1;
                if c < map[h-(a+1)][w] + map[a][w] {
                    c = map[h-(a+1)][w] + map[a][w]
                } 

            }

            while b > 0 {
                b-=1;
                if c < map[h][w-(b+1)] + map[h][b] {
                    c = map[h][w-(b+1)] + map[h][b]
                } 
            }

            map[h][w] = c;
        }
    }

    println!("{}", map[h-1][w-1]);
}
