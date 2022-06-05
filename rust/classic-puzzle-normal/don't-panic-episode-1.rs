use std::{io, collections::HashMap};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let nb_floors = parse_input!(inputs[0], i32); // number of floors
    let width = parse_input!(inputs[1], i32); // width of the area
    let nb_rounds = parse_input!(inputs[2], i32); // maximum number of rounds
    let exit_floor = parse_input!(inputs[3], i32); // floor on which the exit is found
    let exit_pos = parse_input!(inputs[4], i32); // position of the exit on its floor
    let nb_total_clones = parse_input!(inputs[5], i32); // number of generated clones
    let nb_additional_elevators = parse_input!(inputs[6], i32); // ignore (always zero)
    let nb_elevators = parse_input!(inputs[7], i32); // number of elevators

    struct Elevator {
        floor: i32,
        pos: i32,
    }
    let mut hm_elevators = HashMap::<i32, Elevator>::new();

    eprintln!("num {}", nb_elevators);
    for i in 0..nb_elevators as i32 {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let elevator_floor = parse_input!(inputs[0], i32); // floor on which this elevator is found
        let elevator_pos = parse_input!(inputs[1], i32); // position of the elevator on its floor
        let elevator = Elevator {
            floor: elevator_floor,
            pos: elevator_pos,
        };
        eprintln!("i {}", i);
        hm_elevators.insert(i, elevator);
    }


    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let clone_floor = parse_input!(inputs[0], i32); // floor of the leading clone
        let clone_pos = parse_input!(inputs[1], i32); // position of the leading clone on its floor
        let direction = inputs[2].trim().to_string(); // direction of the leading clone: LEFT or RIGHT

        let mut current_elevator_pos: i32 = -1;
        for elevator in hm_elevators.values() {
            if elevator.floor == clone_floor {
                current_elevator_pos = elevator.pos;
            }
        }

        if clone_floor == -1 {
            println!("WAIT");
        }
        if clone_floor == 0 {
            // avoid walls
            if clone_pos == 0 || clone_pos == (width -1) {
                println!("BLOCK");
            } else {
                println!("WAIT");
            }
        } else if current_elevator_pos > 0 {
            // elevator, smart blocking
            if direction == "LEFT" {
                if clone_pos < current_elevator_pos {
                    println!("BLOCK");
                } else {
                    println!("WAIT");
                }
            } else if direction == "RIGHT" {
                if clone_pos > current_elevator_pos {
                    println!("BLOCK");
                } else {
                    println!("WAIT");
                }
            }
        } else if current_elevator_pos < 0 {
            //no elevator avoid walls
            if direction == "LEFT" {
                if clone_pos < exit_pos {
                    println!("BLOCK");
                } else {
                    println!("WAIT");
                }
            } else if direction == "RIGHT" {
                if clone_pos > exit_pos {
                    println!("BLOCK");
                } else {
                    println!("WAIT");
                }
            }
        }
    }
}
