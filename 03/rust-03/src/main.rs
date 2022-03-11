use std::env;
use rust_aoc;

fn main() {
    let input_lines = rust_aoc::get_default_input_lines(Some(env::args().collect()));

    let line_length = input_lines[0].len();
    // check that all lines have the same length
    assert!(input_lines.iter().all(|l| l.len() == line_length));

    let input_values: Vec<usize> = input_lines.iter().map(|l| usize::from_str_radix(l, 2).unwrap()).collect();
    p1(&input_values, line_length);
    p2(&input_values, line_length);
}

/// Get the count of items in `values` that match `bitmask`
fn get_count_match_bitmask(values: &[usize], bitmask: usize) -> usize {
    values.iter().filter(|&v| v & bitmask != 0).count()
}

fn p1(values: &[usize], line_length: usize) {
    let mut ones_count = Vec::<usize>::new();
    let mut bitmask: usize = 1;
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

fn p2(values: &[usize], line_length: usize) {
    let oxy_rating = find_oxy_rating(&values, line_length);
    let co2_rating = find_co2_rating(&values, line_length);
    println!("--- Part II ---");
    println!("Oxygen generator rating: {}", oxy_rating);
    println!("CO2 scrubber rating: {}", co2_rating);
    println!("Life support rating: {}", oxy_rating * co2_rating);
}

fn find_oxy_rating(values: &[usize], line_length: usize) -> usize {
    let mut bitmask: usize = 1 << (line_length - 1);
    let mut oxy_values = values.to_vec();
    for _ in 0..line_length {
        let ones_count = get_count_match_bitmask(&oxy_values, bitmask);
        let ones_count_cutoff = oxy_values.len() / 2 + oxy_values.len() % 2;
        if ones_count >= ones_count_cutoff {
            // ones are more (or equally) common, filter to values matching bitmask
            oxy_values = oxy_values.into_iter().filter(|v| v & bitmask != 0).collect();
        } else {
            // otherwise we filter to values not matching bitmask
            oxy_values = oxy_values.into_iter().filter(|v| v & bitmask == 0).collect();
        }
        if oxy_values.len() == 1 {
            // exit early if just one value remains
            break;
        }
        bitmask = bitmask >> 1;
    }
    // make sure we have only one value left
    assert_eq!(oxy_values.len(), 1);
    return oxy_values[0];
}

fn find_co2_rating(values: &[usize], line_length: usize) -> usize {
    let mut bitmask: usize = 1 << (line_length - 1);
    let mut co2_values = values.to_vec();
    for _ in 0..line_length {
        let ones_count = get_count_match_bitmask(&co2_values, bitmask);
        let ones_count_cutoff = co2_values.len() / 2 + co2_values.len() % 2;
        if ones_count >= ones_count_cutoff {
            // ones are more (or equally) common, filter to values not matching bitmask
            co2_values = co2_values.into_iter().filter(|v| v & bitmask == 0).collect();
        } else {
            // otherwise we filter to values matching bitmask
            co2_values = co2_values.into_iter().filter(|v| v & bitmask != 0).collect();
        }
        if co2_values.len() == 1 {
            // exit early if just one value remains
            break;
        }
        bitmask = bitmask >> 1;
    }
    // make sure we have only one value left
    assert_eq!(co2_values.len(), 1);
    return co2_values[0];
}