use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide a path to the input file");
    }

    let input = &args[1];
    let content = fs::read_to_string(input).expect("Failed to read file");
    let readings: Vec<i32> = content
        .split_ascii_whitespace()
        .map(|r| r.parse::<i32>().unwrap())
        .collect();
    
    let mut increases = 0;
    for r in readings.windows(2) {
        if r[1] > r[0] {
            increases += 1;
        }
    }
    println!("There were {} reading increases", increases);
}
