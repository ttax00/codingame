use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let w = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, i32);

    // build out the map
    let mut map = Vec::<Vec<i32>>::new();
    for _i in 0..h as usize {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        let line = inputs.split_whitespace().map(|x| x.parse::<i32>().expect("should be int")).collect::<Vec<i32>>();
        eprintln!("{:?}", line);
        map.push(line);
    }

    // solve
    let t = find_treasure(&map);
    println!("{} {}", t.1, t.0);

    struct Treasure(i32, i32);
    fn find_treasure(map: &Vec::<Vec<i32>>) -> Treasure {

        for h in 0..map.len()  as i32 {
            for w in 0..map[h as usize].len() as i32 {
                eprintln!("h: {} w: {}", h, w);
                if is_surrounded(&h, &w, map) { 
                    eprintln!("is surrounded! {} {}", h, w);
                    return Treasure(h as i32, w as i32);
                }
            }
        }

        return Treasure(0, 0) // shouldn't be reached
    }

    fn is_surrounded(h: &i32, w: &i32, map: &Vec<Vec<i32>>) -> bool {
        is_obstical(h -1, w -1, map) && is_obstical(*h, w - 1, map) && is_obstical(h + 1, w -1, map)
        && is_obstical(h - 1, *w, map) && is_treasure(*h, *w, map) && is_obstical(h + 1, *w, map)
        && is_obstical(h -1, w + 1, map) && is_obstical(*h, w + 1, map) && is_obstical(h + 1, w + 1, map)
    }

    fn is_treasure(h: i32, w: i32, map: &Vec<Vec<i32>>) -> bool {
        map[h as usize][w as usize] == 0
    }

    fn is_obstical(h: i32, w: i32, map: &Vec<Vec<i32>>) -> bool {

        if h < 0 || h as usize >= map.len() { true }
        else if w < 0 || w as usize >= map[h as usize].len() { true }
        else if map[h as usize][w as usize] == 1 { true }
        else { false }
    }

}
