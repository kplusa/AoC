use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_data_from_txt<P: AsRef<Path>>(filepath: P) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }

    Ok(grid)
}

fn count_xmas(grid: &Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    fn check_direction(
        grid: &Vec<Vec<char>>,
        row: isize,
        col: isize,
        dr: isize,
        dc: isize,
        rows: usize,
        cols: usize,
    ) -> bool {
        let mut word = String::new();
        for i in 0..4 {
            let r = row + i * dr;
            let c = col + i * dc;
            if r >= 0 && (r as usize) < rows && c >= 0 && (c as usize) < cols {
                word.push(grid[r as usize][c as usize]);
            } else {
                return false;
            }
        }
        word == "XMAS"
    }

    let directions = [
        (0, 1),  // Horizontal right
        (0, -1), // Horizontal left
        (1, 0),  // Vertical down
        (-1, 0), // Vertical up
        (1, 1),  // Diagonal down-right
        (-1, -1), // Diagonal up-left
        (1, -1), // Diagonal down-left
        (-1, 1),  // Diagonal up-right
    ];

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                if check_direction(
                    grid,
                    r as isize,
                    c as isize,
                    dr,
                    dc,
                    rows,
                    cols,
                ) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part1(grid: &Vec<Vec<char>>) {
    let total_xmas = count_xmas(grid);
    println!("Part 1: The word 'XMAS' appears {} times in the word search.", total_xmas);
}

pub fn part2(grid: &Vec<Vec<char>>) {
    
}

pub fn run() {
    match read_data_from_txt("src/assets/day4/xmas.txt") {
        Ok(grid) => {
            part1(&grid);
            println!("");
            part2(&grid);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}