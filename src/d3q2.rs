use std::fs;
use std::ops::Index;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle, ParallelProgressIterator};

pub fn d3q2() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    let strings = fs::read_to_string("d3q1.txt")
        .expect("Should have been able to read file");

    let lines: Vec<&str> = strings.lines().collect();

    let bar = ProgressBar::new(lines.len() as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})"
        ).unwrap()
    );

    let result: u64 = lines
        .par_iter()
        .progress_with(bar)
        .map(|line| max_joltage_for_bank(line.to_string()))
        .sum();

    println!("{result}");
}

fn max_joltage_for_bank(line: String) -> u64 {
    let mut count = 0;
    let chars: Vec<char> = line.chars().collect();
    let mut current_chars = chars.clone();
    let mut lower_bound = 0;
    for i in 0..12 {
        let legal_region = &current_chars[lower_bound..(chars.len() - (12 - i - 1))];
        let max = legal_region
            .iter().map(|c| c.to_digit(10).unwrap() as u64).max().unwrap_or(0);
        lower_bound += legal_region
            .iter().position(|c| c.to_digit(10).unwrap() as u64 == max).unwrap_or(lower_bound) + 1;
        count += max * u64::pow(10, (11 - i) as u32)
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(max_joltage_for_bank("987654321111111".to_string()), 987654321111);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(max_joltage_for_bank("234234234234278".to_string()), 434234234278);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(max_joltage_for_bank("818181911112111".to_string()), 888911112111);
    }
}