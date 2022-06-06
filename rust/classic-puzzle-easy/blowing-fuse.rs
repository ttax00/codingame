use std::io;
use std::collections::HashMap;

fn main() {
    // line 1
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let n = inputs[0].trim().parse::<i32>().unwrap();
    let m = inputs[1].trim().parse::<i32>().unwrap();
    let c = inputs[2].trim().parse::<i32>().unwrap();
    // line 2
  
    struct Device {
        on: bool,
        consumption: i32
    }
    let mut devices = HashMap::<i32, Device>::new();

    let mut inputs = String::new();
    let mut id = 1;
    io::stdin().read_line(&mut inputs).unwrap();
    for i in inputs.split_whitespace() {
        let nx = i.trim().parse::<i32>().unwrap();
        devices.insert(id, Device{
            on: false,
            consumption: nx,
        });
        id = id + 1;
    }

    //line 3
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let mut max : i32 = 0;
    for i in inputs.split_whitespace() {
        let device_id = i.trim().parse::<i32>().unwrap();
        let device = devices.get_mut(&device_id).unwrap();
        device.on = !device.on;

        let total = cal_total(&devices);
        eprintln!("{}-{}", total, max);
        if total > max {
            max = total
        }

        if max > c {
            println!("Fuse was blown.");
            break;
        } 
    }

    if max <= c {
        println!("Fuse was not blown.");
        println!("Maximal consumed current was {} A.", max);
    }

    fn cal_total(devices: &HashMap::<i32, Device>) -> i32 {
        let mut total: i32 = 0;
        for device in devices.values() {
            if device.on {
                total += device.consumption;
            }
        }
        return total;
    }

}
