use std::fs;
use std::str::FromStr;

pub fn d3q1() {
    let strings = fs::read_to_string("d3q1.txt")
        .expect("Should have been able to read file");
    println!("{}", strings.lines().map(|line| max_joltage_for_bank(line.to_string())).sum::<u32>());
}

fn max_joltage_for_bank(line: String) -> u32 {
    let mut max = 0;
    for (i, c1) in line.chars().enumerate() {
        for (j, c2) in line.chars().enumerate() {
            if i >= j {
                continue;
            }
            let value = u32::from_str(&*(c1.to_string() + &*c2.to_string())).expect("Should be a number");
            if (value > max) {
                max = value;
            }
        }
    }
    return max;
}