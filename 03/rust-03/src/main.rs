use rust_aoc;

fn main() {
    let input_lines = rust_aoc::get_default_input_lines();

    let line_length = input_lines[0].len();
    // check that all lines have the same length
    assert!(input_lines.iter().all(|l| l.len() == line_length));

    let input_values: Vec<usize> = input_lines.iter().map(|l| usize::from_str_radix(l, 2).unwrap()).collect();
    p1(&input_values, line_length);
}

fn get_count_match_bitmask(values: &[usize], bitmask: usize) -> usize {
    values.iter().filter(|&v| v & bitmask > 0).count()
}

fn p1(values: &[usize], line_length: usize) {
    let mut ones_count = Vec::<usize>::new();
    let mut bitmask = 1;
    for _ in 0..line_length {
        ones_count.push(get_count_match_bitmask(values, bitmask));
        bitmask = bitmask << 1;
    }

    // build binary strings for the gamma and epsilon values
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();

    // reverse ones_count vector so that we start with most significant bit and can append to strings
    for c in ones_count.into_iter().rev() {
        if c > values.len() / 2 {
            // more than half of bits in this position were 1s
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            // more than half of bits in this position were 0s
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }

    let gamma_rate = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_str, 2).unwrap();

    println!("--- Part I ---");
    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Power consumption: {}", gamma_rate * epsilon_rate);
}