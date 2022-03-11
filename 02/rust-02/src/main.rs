use std::{fs, env};

fn main() {
    let lines = get_input_lines(&get_input_path());
    let instructions = lines
        .iter()
        .map(|l| string_to_instruction(l));
    let mut pos = 0;
    let mut depth = 0;
    for inst in instructions {
        match inst {
            Instruction::Forward(units) => pos += units,
            Instruction::Up(units) => depth -= units,
            Instruction::Down(units) => depth += units
        }
    }
    println!("Position: {}", pos);
    println!("Depth: {}", depth);
    println!("Product: {}", pos * depth);
}

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return "../input.txt".to_string();
    } else {
        return args[1].to_string();
    }
}

fn get_input_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap().lines().map(|l| l.to_string()).collect()
}

fn string_to_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let units = parts[1].parse::<i32>().unwrap();
    match parts[0] {
        "forward" => Instruction::Forward(units),
        "up" => Instruction::Up(units),
        "down" => Instruction::Down(units),
        other => panic!("Unknown instruction '{}'", other)
    }
}


// The enum can store both the instruction type and the associated data
enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32)
}