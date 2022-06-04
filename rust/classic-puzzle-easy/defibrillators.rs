use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lon = parse_input!(input_line.replace(',',"."), f32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lat = parse_input!(input_line.replace(',',"."), f32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    

    fn cal_distance(lon: f32, lat: f32, d_lon: f32, d_lat: f32) -> f32 {
        let x = (d_lon - lon)* ((lat + d_lat ) / 2.0).cos();
        let y = d_lat - lat;
        return (x*x + y*y).sqrt() * 6371.0;
    }

    let mut dist = -1.0;
    let mut current: String = String::new();
    
    for _i in 0..n as usize {

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let defib = input_line.trim_matches('\n').to_string();
        let info: Vec<&str> = defib.split(';').collect();
        let d_lon = info[4].replace(',',".").parse::<f32>().unwrap();
        let d_lat = info[5].replace(',',".").parse::<f32>().unwrap();
        let d_name = info[1];

        if dist == -1.0 {
            dist = cal_distance(lon, lat, d_lon, d_lat);
            current = d_name.to_string();
        }


        let test = cal_distance(lon, lat, d_lon, d_lat);
        if test < dist {
            dist = test;
            current = d_name.to_string();
        }

    }

    println!("{}", current);
}
