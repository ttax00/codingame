use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut players = Vec::<_>::new();
    for _i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let player = input_line.trim_matches('\n').to_string();
        players.push(player);
    }

    let mut fewest_rounds = i32::MAX;
    let mut winner = String::from(&players[players.len()-1]);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let shoots = input_line.trim_matches('\n').to_string();
        let rounds = simulate_game(&mut shoots.split_whitespace().collect());


        if rounds != -1 && rounds < fewest_rounds {
            fewest_rounds = rounds;
            winner = String::from(&players[i]);
        } 
    }
    println!("{}", winner);

    fn simulate_game(game: &mut Vec<&str>) -> i32 {
        let mut total_point = 0;
        let mut round = 0;
        while game.len() > 0 {
            round +=1;
            eprintln!("game: {} / {}", game.len(), round);
            let mut misses = [0, 0, 0];
            for i in 0..3 {
                if game.len() == 0 {
                    break;
                }
                let string = game.remove(0);
                let mut points_to_add = 0;
                match string.parse::<i32>() {
                    Ok(p) => {
                        points_to_add += p
                    },
                    Err(_) => match string {
                        "X" => {
                            misses[i] = 1;
                            match misses {
                                [1,0,0] => points_to_add -= 20,
                                [0,1,0] => points_to_add -= 20,
                                [0,0,1] => points_to_add -= 20,
                                [1,0,1] => points_to_add -= 20,
                                [1,1,0] => points_to_add -= 30,
                                [0,1,1] => points_to_add -= 30,
                                [1,1,1] => {
                                    total_point = 0;
                                    points_to_add = 0
                                },
                                [_,_,_] => unreachable!(),
                            }
            
                        },
                        s => {
                            // A*B case
                            let multi = s.split("*")
                                            .map(|s|s.parse::<i32>().unwrap())
                                            .collect::<Vec<i32>>();
                            points_to_add += multi[0] * multi[1];
                        }
                    }
                }
                eprintln!("{}", total_point + points_to_add);
                if total_point + points_to_add > 101 {
                    eprintln!("over score");
                    break;
                }  else if total_point + points_to_add == 101{
                    eprintln!("winner!");
                    return round;
                } else {
                    total_point += points_to_add;

                }
            }
        }
        -1 //looses
    }   
}
