use crate::d7q2::Symbol::SPLIT;
use std::cmp::PartialEq;
use std::fs;

pub fn d7q2() {
    let strings = fs::read_to_string("d7q1.txt")
        .expect("Should have been able to read file");

    let mut lines: Vec<Vec<Symbol>> = strings.lines().map(
        |line| line.to_ascii_lowercase().as_bytes().iter().map(
            |&byte| {
                match byte {
                    b's' => Symbol::START,
                    b'^' => Symbol::SPLIT,
                    b'.' => Symbol::EMPTY,
                    b'|' => Symbol::PIPE { count: 1 },
                    _ => panic!("Invalid character: {}", byte as char),
                }
            }).collect()
    ).collect();
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let symbol = lines[i][j];
            if i + 1 < lines.len() {
                match symbol {
                    Symbol::START => {
                        lines[i + 1][j] = Symbol::PIPE { count: 1 };
                    }
                    SPLIT => {}
                    Symbol::EMPTY => {}
                    Symbol::PIPE { count: previous_count } => {
                        if lines[i + 1][j] == SPLIT {
                            if j + 1 < lines[i].len() {
                                let right = lines[i + 1][j + 1];
                                let current_right_count = match right {
                                    Symbol::PIPE { count: right_count } => right_count,
                                    _ => 0,
                                };
                                lines[i + 1][j + 1] = Symbol::PIPE { count: previous_count + current_right_count };
                            }
                            if (j as isize - 1) >= 0 {
                                let left = lines[i + 1][j - 1];
                                let current_left_count = match left {
                                    Symbol::PIPE { count: left_count } => left_count,
                                    _ => 0,
                                };
                                lines[i + 1][j - 1] = Symbol::PIPE { count: previous_count + current_left_count };
                            }
                        }
                        else {
                            let old = lines[i + 1][j];
                            let old_count = match old {
                                Symbol::PIPE { count: old_count } => old_count,
                                _ => 0,
                            };
                            lines[i + 1][j] = Symbol::PIPE { count: old_count + previous_count };
                        }
                    }
                }
            }
        }
    }
    let universes = lines.last().unwrap().iter().map(|it| {
        match it {
            Symbol::PIPE { count } => *count,
            _ => 0u64
        }
    }).reduce(|a, b| {
        a + b
    }).unwrap();
    for line in lines {
        println!("{:?}", line);
    }
    println!("{}", universes);
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Symbol {
    EMPTY,
    SPLIT,
    START,
    PIPE {count: u64}
}

/*
pub fn recursive_d7q2(i: usize, splits: u64, lines: &mut Vec<Vec<u8>>) -> u64 {
    let mut local_splits = splits;
    for j in 0..lines[i].len() {
        let char = lines[i][j] as char;
        if i + 1 < lines.len() {
            match char {
                's' => {
                    let before = lines[i + 1][j];
                    lines[i + 1][j] = '|' as u8;
                    local_splits += recursive_d7q2(i + 1, splits, lines);
                    lines[i + 1][j] = before;
                }
                '^' => {}
                '.' => {}
                '|' => {
                    if lines[i + 1][j] == '^' as u8 {
                        if j + 1 < lines[i].len() {
                            let before = lines[i + 1][j];
                            lines[i + 1][j + 1] = '|' as u8;
                            local_splits += recursive_d7q2(i + 1, splits, lines);
                            lines[i + 1][j + 1] = before;
                        }
                        if (j as isize - 1) >= 0 {
                            let before = lines[i + 1][j - 1];
                            lines[i + 1][j - 1] = '|' as u8;
                            local_splits += recursive_d7q2(i + 1, splits, lines);
                            lines[i + 1][j - 1] = before;
                        }
                    }
                    else {
                        let before = lines[i + 1][j];
                        lines[i + 1][j] = '|' as u8;
                        local_splits += recursive_d7q2(i + 1, splits, lines);
                        lines[i + 1][j] = before;
                    }
                }
                _ => {
                    panic!("Invalid character: {}", char);
                }
            }
        }
    }
    if (local_splits > 1000000) {
        println!("Split count: {}, i: {}", local_splits + 1, i);
    }
    local_splits + 1
}

 */