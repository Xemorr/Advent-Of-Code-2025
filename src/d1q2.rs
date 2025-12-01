use std::fs;
use std::str::FromStr;

pub fn d1q2() {
    let strings = fs::read_to_string("d1q1.txt")
        .expect("Should have been able to read file");

    let mut position = 50;
    let mut at_zero_count = 0;
    for line in strings.lines() {
        let split = line.split_at(1);
        println!("{:?}", split);
        let mut sign = 1;
        if split.0 == "L" {
            sign = -1;
        }
        let number = i64::from_str(split.1).unwrap();
        for _ in 0..number {
            position = (position + sign) % 100;
            if (position < 0) {
                position += 100;
            }
            if (position == 0) {
                at_zero_count += 1;
            }
        }
    }

    println!("{}", at_zero_count);
}