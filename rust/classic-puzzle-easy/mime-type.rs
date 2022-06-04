use std::{io, collections::HashMap};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // Number of elements which make up the association table.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let q = parse_input!(input_line, i32); // Number Q of file names to be analyzed.
    let mut association_table = HashMap::new();

    for _i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let ext = inputs[0].trim().to_string(); // file extension
        let mt = inputs[1].trim().to_string(); // MIME type.
        association_table.insert(ext.to_lowercase(), mt);
    }
    for _i in 0..q as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let fname = input_line.trim_matches('\n').to_string().to_lowercase();
         // One file name per line.
        let ext = fname.split('.').last().unwrap();
        let t = association_table.get(ext);

        if fname.split('.').count() < 2 || t == None {
            println!("UNKNOWN");
        } else {
            println!("{}", t.unwrap());
        }

    }

    // For each of the Q filenames, display on a line the corresponding MIME type. If there is no corresponding type, then display UNKNOWN.
}