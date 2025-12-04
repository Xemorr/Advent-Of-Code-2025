use std::fs;

pub fn d4q1() {
    let strings = fs::read_to_string("d4q1.txt")
        .expect("Should have been able to read file");

    let mut count = 0;
    let grid: Vec<Vec<char>> = strings.lines().map(|line| line.chars().collect()).collect();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                if count_neighbours(&grid, i, j) >= 4 {
                    continue
                }
                count += 1
            }
        }
    }

    println!("{}", count);
}

fn count_neighbours(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for i in -1isize..=1 {
        for j in -1isize..=1 {
            if i == 0 && j == 0 { continue; }
            let x: isize = x.try_into().unwrap();
            let y: isize = y.try_into().unwrap();
            let x_to_try = (i + x);
            let y_to_try = (j + y);
            if (x_to_try < 0) {
                continue;
            }
            if (x_to_try >= (grid.len() as isize)) {
                continue;
            }
            if (y_to_try < 0) {
                continue;
            }
            if (y_to_try >= (grid[x_to_try as usize].len() as isize)) {
                continue;
            }
            if grid[x_to_try as usize][y_to_try as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}