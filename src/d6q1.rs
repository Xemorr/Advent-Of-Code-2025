use std::fs;
use rayon::iter::IntoParallelRefIterator;

pub fn d6q1() {
    let strings = fs::read_to_string("d6q1.txt")
        .expect("Should have been able to read file");
    
    let tokens_per_line: Vec<Vec<&str>> = strings.lines()
        .map(|line| line.split(" ").filter(|word| !word.is_empty()).collect())
        .collect();
    
    let mut columns: Vec<Vec<&str>> = Vec::new();
    for (i, line) in tokens_per_line.iter().enumerate() {
        for (j, token) in line.iter().enumerate() {
            let mut x = columns.get_mut(j);
            match x {
                Some(x) => {
                    x.push(token);
                },
                None => {
                    columns.push(vec![token]);
                },
            };
        }
    }

    let grand_total: u64 = columns.iter().map(|column| {
        let operator = column.last();
        let result = column[..(column.len() - 1)].iter()
            .map(|it| it.parse::<u64>().unwrap())
            .reduce(|a, b| {
                match operator {
                    Some(&"+") => a + b,
                    Some(&"-") => a - b,
                    Some(&"*") => a * b,
                    Some(&"/") => a / b,
                    _ => panic!("Invalid operator"),
                }
            }).unwrap();
        result
    }).reduce(|a, b| a + b).unwrap();
    println!("{:#?}", grand_total);
}