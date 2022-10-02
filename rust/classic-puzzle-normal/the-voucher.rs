use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let v = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut products: Vec<i32> = vec![]; 

    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let fname = input_line.trim().split(" ").collect::<Vec<_>>();
        let price = fname.last().unwrap().parse::<i32>().unwrap();
        products.push(price);
    }

    fn find_comb(total: i32, voucher: i32, index: usize, products: &Vec<i32>) -> i32 {
        if index >= products.len() { return 0; }
        let product = products[index];
        let mut comb = 0;
        for i in 0..=3 {
            let total = total + product * i;
            if total < voucher {
                comb += find_comb(total, voucher, index + 1, products);
            } else if total > voucher {
                return comb;
            } else if total == voucher {
                comb += 1;
                return comb;
            }
        }
        comb
    }

    println!("{}", find_comb(0, v, 0, &products));
}
