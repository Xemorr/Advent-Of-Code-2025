use std::ffi::c_long;
use std::fs;
use std::str::FromStr;
use rayon::iter::IntoParallelRefIterator;

pub fn d6q2() {
    let strings = fs::read_to_string("d6q1.txt")
        .expect("Should have been able to read file");
    
    let lines: Vec<&str> = strings.lines().collect();
    let mut character_column = Vec::new();
    for i in 0..lines[0].len() {
        character_column.push(
            lines[0].chars().nth(i).unwrap().to_string() +
                lines[1].chars().nth(i).unwrap().to_string().as_str() +
                lines[2].chars().nth(i).unwrap().to_string().as_str() +
                lines[3].chars().nth(i).unwrap().to_string().as_str()
            );
    }
    let mut operators = Vec::new();
    for i in 0..lines[0].len() {
        let x = lines[4].chars().nth(i);
        if x.is_none() { continue }
        let x = x.unwrap();
        if (x.is_whitespace()) { continue }
        operators.push(x);
    }

    let mut column_iterator = character_column.iter();
    let mut operator_iterator = operators.iter();
    let mut current_column = "";
    let mut current_operator = operator_iterator.next().unwrap();
    let mut current_value = match current_operator {
        '+' => 0u64,
        '*' => 1u64,
        _ => panic!("Invalid operator"),
    };
    let mut total = 0u64;
    loop {
        let current_column_maybe = column_iterator.next();
        if current_column_maybe.is_none() {
            total += current_value;
            break
        }
        current_column = current_column_maybe.unwrap();
        current_column = current_column.trim();
        while (current_column.is_empty()) {
            total += current_value;
            current_operator = operator_iterator.next().unwrap_or_else(|| &'+');
            current_value = match current_operator {
                '+' => 0u64,
                '*' => 1u64,
                _ => panic!("Invalid operator"),
            };
            let current_column_maybe = column_iterator.next();
            if current_column_maybe.is_none() {
                break
            }
            current_column = current_column_maybe.unwrap();
            current_column = current_column.trim();
        }
        current_value = match current_operator {
            '+' => current_value + u64::from_str(current_column).unwrap(),
            '*' => current_value * u64::from_str(current_column).unwrap(),
            _ => panic!("Invalid operator"),
        };
    }
    println!("{}", total);
}