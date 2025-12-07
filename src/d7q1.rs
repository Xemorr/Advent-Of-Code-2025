use std::fs;
use std::ops::Index;

pub fn d7q1() {
    let strings = fs::read_to_string("d7q1.txt")
        .expect("Should have been able to read file");

    let mut lines: Vec<Vec<u8>> = strings.lines().map(|line| line.to_ascii_lowercase().as_bytes().to_vec()).collect();
    let mut splits = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let char = lines[i][j] as char;
            if i + 1 < lines.len() {
                match char {
                    's' => {
                        lines[i + 1][j] = '|' as u8;
                    }
                    '^' => {}
                    '.' => {}
                    '|' => {
                        if lines[i + 1][j] == '^' as u8 {
                            splits += 1;
                            if j + 1 < lines[i].len() { lines[i + 1][j + 1] = '|' as u8; }
                            if (j as isize - 1) >= 0 { lines[i + 1][j - 1] = '|' as u8; }
                        }
                        else {
                            lines[i + 1][j] = '|' as u8;
                        }
                    }
                    _ => {
                        panic!("Invalid character: {}", char);
                    }
                }
            }
        }
    }
    for line in &lines {
        println!("{}", String::from_utf8_lossy(line));
    }
    println!("{}", splits);
}