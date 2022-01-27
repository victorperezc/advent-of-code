use std::{fs,env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let depths : Vec<&str> = contents.split("\n").collect();
    let mut count : u32 = 0;
    for i in 1..depths.len() {
        if depths[i-1] < depths[i] {
            count = count + 1;
        }
    }

    println!("How many measurements are larger than the previous measurement?");
    println!("There are {} measurements larger than the previous", count);    
}
