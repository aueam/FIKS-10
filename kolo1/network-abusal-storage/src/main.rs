use std::collections::HashMap;
use std::fs::read_to_string;

const DIVIDE_BY: i32 = 110;

fn get_dividers(numaa: i32) -> Vec<i32> {
    let mut dividers = Vec::new();
    for i in 1..DIVIDE_BY {
        if numaa % i == 0 { dividers.push(i); }
    }
    dividers
}

fn find_biggest_divider(dividers: HashMap<i32, i32>) -> i32 {
    let mut number_of_numbers = 0;

    for (divider, d_number_of_numbers) in dividers.iter() {
        if divider > &number_of_numbers {
            if divider == d_number_of_numbers {
                // save
                number_of_numbers = *divider;
            } else {
                for another_divider in &get_dividers(*divider) {
                    if another_divider <= d_number_of_numbers && another_divider > &number_of_numbers {
                        // save
                        number_of_numbers = *another_divider;
                    }
                }
            }
        }
    }

    number_of_numbers
}

fn main() {
    let mut is_latency = false;
    for (index, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        if index == 0 { continue; }
        if !is_latency { is_latency = true; } else {
            is_latency = false;

            let latency: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| { s.parse().ok() })
                .collect();

            let mut dividers: HashMap<i32, i32> = HashMap::new();

            for one_latency in latency {
                for d in get_dividers(one_latency) {
                    let count = dividers.entry(d).or_insert(0);
                    *count += 1;
                }
            }

            println!("{}", find_biggest_divider(dividers));
        }
    }
}
