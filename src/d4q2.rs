use std::fs;

pub fn d4q2() {
    let strings = fs::read_to_string("d4q1.txt")
        .expect("Should have been able to read file");
    let mut outer_count = 0;

    let mut count = 1;
    let mut grid: Vec<Vec<char>> = strings.lines().map(|line| line.chars().collect()).collect();
    while (count != 0) {
        count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '@' {
                    if count_neighbours(&grid, i, j) >= 4 {
                        continue
                    }
                    grid[i][j] = 'x';
                    count += 1;
                }
            }
        }
        outer_count += count;
    }

    println!("{}", outer_count);
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