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
        .map(|r| r.parse::<i32>().unwrap()) // this assumes the file contains valid input
        .collect();
    
    let single_increases = calc_increases(&readings);
    println!("There were {} single reading increases", single_increases);

    let window_sums = sum_windows(&readings);
    let window_increases = calc_increases(&window_sums);
    println!("There were {} sliding window increases", window_increases);
}

fn sum_windows(readings: &[i32]) -> Vec<i32> {
    readings
        .windows(3)
        .map(|w| w.iter().sum())
        .collect()
}

fn calc_increases(readings: &[i32]) -> i32 {
    let mut increases = 0;
    for r in readings.windows(2) {
        if r[1] > r[0] {
            increases += 1;
        }
    }
    return increases
}