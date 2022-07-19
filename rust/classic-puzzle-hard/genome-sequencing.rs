use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let sub_sequences = (0..n).map(|_| {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line.trim().to_string()
    }).collect::<Vec<String>>();

    fn permutate<'a>(sequences: &Vec<&'a str>) -> Vec<Vec<&'a str>>{
        let mut permutations = Vec::<Vec<&'a str>>::new();
        for x in 0..sequences.len() {
            let sequence = sequences[x];
            for i in 0..sequences.len() {
                // if i == x {continue;}
                let mut permutation = sequences.iter().filter(|s| s != &&sequence).map(|s| *s).collect::<Vec<&str>>();
                if permutation.len() == 0 {
                    permutation = sequences.to_vec();
                }
                permutation.insert(i, sequence);
                permutations.push(permutation);
            }
        }
        permutations
    }
    let test = permutate(&sub_sequences.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    eprintln!("{:?}", test);
    let mut lengths = test.iter().map(|p| {
        let mut string = String::from(p[0]);
        for ip in 1..p.len() {
            for is in 0..string.len() {
                let next = p[ip];
                eprintln!("{} / {}", string, next);
                if string.contains(next) {
                    break;
                } else if string.len() - is > next.len() {
                    continue;
                } else {
                    if string[is..] == next[0..string.len() - is] {
                        string.push_str(&next[string.len() - is..]);
                        break;
                    }
                }

                if is == string.len() -1 {
                    string.push_str(next);
                }
            }
        }

        string.len() as i32
    }).collect::<Vec<i32>>();
    lengths.sort();
    eprintln!("{:?}", lengths);
    println!("{}", lengths[0]);
}
_