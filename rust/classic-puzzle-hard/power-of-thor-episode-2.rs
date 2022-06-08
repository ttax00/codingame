use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let mut tx = parse_input!(inputs[0], i32);
    let mut ty = parse_input!(inputs[1], i32);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let _ = parse_input!(inputs[0], i32); // the remaining number of hammer strikes.
        let n = parse_input!(inputs[1], i32); // the number of giants which are still present on the map.
        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut dist = Vec::<(f64, i32, i32, i32, i32)>::new();
        let get_dist = |ax: &i32, ay: &i32, bx: &i32, by: &i32| -> f64 {
            (((ax - bx).pow(2) + (ay - by).pow(2)) as f64).sqrt() 
        };
        for _i in 0..n as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            dist.push((get_dist(&x, &y, &tx, &ty) , x - tx, y-ty, x, y));
        }
        dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let center_x = (max_x - min_x) / 2 + min_x;
        let center_y = (max_y - min_y) / 2 + min_y;
        eprintln!("center: {} {}", center_x, center_y);
        eprintln!("thor: {} {}", tx, ty);
        eprintln!("dist  {:?}", dist);

        let mut valid_moves = Vec::<(i32, i32, f64, f64)>::new();
        for dx in [-1, 0,1].iter() {
            for dy in [-1, 0, 1].iter() {
                if dx == &0 && dy == &0 { continue;}
                let x = tx + dx;
                let y = ty + dy;
                let (_, _, _, mx, my) = dist[0];
                let dist_nearest = get_dist(&x, &y, &mx, &my);
                let dist_center = get_dist(&x, &y, &center_x, &center_y);
                if (x - mx).abs() <=2 && (y - my).abs() <=2 {
                    continue; // not valid;
                }
                if x < 0 || x >= 40 || y < 0 || y >= 18 {
                    continue;
                }

                valid_moves.push((*dx,*dy, dist_nearest, dist_center));
            }
        }
        eprintln!("valids: {:?}", valid_moves);
        if valid_moves.len() == 0 {
            println!("STRIKE");
        } else {
            let mut toward_center_moves = valid_moves.iter()
                                .filter(|(_,_,_,c)| c <= &(get_dist(&tx, &ty, &center_x, &center_y)+0.2))
                                .collect::<Vec<&(i32, i32, f64, f64)>>(); 
            if toward_center_moves.len() == 0 {
                println!("WAIT");
                continue;
            }
            //sort for furthest away from nearest enemy
            toward_center_moves.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
            eprintln!("{:?}", toward_center_moves);

            let (dx, dy, _, _) = toward_center_moves[toward_center_moves.len()-1];
            tx += dx;
            ty += dy;

            eprintln!("{:?}", (dx, dy));
            match (dx, dy) {
                (0, 0) => println!("WAIT"),
                (1, 1) => println!("SE"),
                (1, 0) => println!("E"),
                (0, 1) => println!("S"),
                (-1, -1) => println!("NW"),
                (-1, 0) => println!("W"),
                (0, -1) => println!("N"),
                (1, -1) => println!("NE"),
                (-1, 1) => println!("SW"),
                (_, _) => unreachable!(),
            }
        }

    }
}
