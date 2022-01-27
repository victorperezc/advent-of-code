use std::{fs,env};

fn main() {
    let window_size: usize = 3;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let depths : Vec<&str> = contents.split("\n").collect();
    let mut windows : Vec<u32> = Vec::new();

    for i in 0..depths.len() {
        let max: usize = depths.len() - window_size;
        if i < max {
            let mut window: u32 = 0;
            for j in 0..window_size {
                let value: u32 = depths[i+j].parse::<u32>().unwrap();
                window = window + value;
            }
            windows.push(window);
        }
    }

    let mut count : u32 = 0;
    for i in 1..windows.len() {
        if windows[i-1] < windows[i] {
            count = count + 1;
        }
    }
    println!("{}",depths.len());
    println!("{}",windows.len());
    println!("How many measurements are larger than the previous measurement?");
    println!("There are {} measurements larger than the previous", count);
}
