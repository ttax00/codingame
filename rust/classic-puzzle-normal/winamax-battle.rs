use std::{io, collections::VecDeque};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of cards for player 1
    let mut q1 = VecDeque::<String>::new();
    for _i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let cardp_1 = input_line.trim().to_string(); // the n cards of player 1
        q1.push_back(cardp_1);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = parse_input!(input_line, i32); // the number of cards for player 2
    let mut q2 = VecDeque::<String>::new();
    for _i in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let cardp_2 = input_line.trim().to_string(); // the m cards of player 2
        q2.push_back(cardp_2);
    }

    fight(&mut q1, &mut q2);

    // struct Game(i32, i32);
    fn fight(q1: &mut VecDeque<String>, q2: &mut VecDeque::<String>) -> i32 {
        let mut round = 0;
        let mut war = 0;

        while q1.len() > 0 && q2.len() > 0 {
            eprintln!("p1: {:?}", q1);
            eprintln!("p2: {:?}", q2);
            let play1 = q1.pop_front().unwrap();
            let play2 = q2.pop_front().unwrap();
            let i1 = get_index(&play1);
            let i2 = get_index(&play2);
            let mut v1 = vec![play1];
            let mut v2 = vec![play2];

            
            if i1 == i2 {
                // war
                // PAT exception
                eprintln!("war");
                war += 1;
                round -=1;
                if q1.len() < 4 || q2.len() < 4 {
                    println!("PAT");
                    break;
                }

                (0..3).for_each(|_| {
                    v1.push(q1.pop_front().unwrap());
                    v2.push(q2.pop_front().unwrap());
                });
            }

            if i1 > i2 {

                q1.append(&mut VecDeque::from(v1));

                q1.append(&mut VecDeque::from(v2));
            } else {

                q2.append(&mut VecDeque::from(v1));

                q2.append(&mut VecDeque::from(v2));
            }
            round += 1;
        }
        #[derive(Debug)]
        struct A(usize, usize);
        eprintln!("{:?}", A(q1.len(), q2.len()));
        eprintln!("War {}", war);
        match A(q1.len(), q2.len()) {
            A(0, 0) => println!("PAT"),
            A(_, 0) => println!("{} {}", 1, round),
            A(0, _) => println!("{} {}", 2, round),
            A(_,_) => {},
        }
        round
    }

    assert_eq!(get_index("3S"), 2);
    fn get_index(card: &str) -> usize {
        let card = &card[0..card.len()-1];
        let order = ["UNKNOWN","2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        order.iter().position(|c| c == &card).unwrap_or_else(|| {0})
        
    }
}

