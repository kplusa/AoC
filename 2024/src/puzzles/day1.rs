use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_data_from_txt<P: AsRef<Path>>(filepath: P) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let left: i32 = parts[0]
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let right: i32 = parts[1]
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            left_values.push(left);
            right_values.push(right);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid line format",
            ));
        }
    }

    Ok((left_values, right_values))
}

pub fn part1(left: &mut Vec<i32>, right: &mut Vec<i32>) {
    left.sort();
    right.sort();

    if left.len() != right.len() {
        eprintln!("Error: Left and right lists must have the same length.");
        return;
    }

    let mut subtraction_results = Vec::new();
    let mut sum_of_subtractions: i32 = 0;

    for i in 0..left.len() {
        let subtraction = (left[i] - right[i]).abs();
        subtraction_results.push(subtraction);
        sum_of_subtractions += subtraction;
    }

    // println!("Part 1 - Left: {:?}", left);
    // println!("Part 1 - Right: {:?}", right);
    // println!("Part 1 - Subtraction Results (Absolute): {:?}", subtraction_results);
    println!("Part 1 - Sum of Absolute Differences: {}", sum_of_subtractions);
}

pub fn part2(left: &Vec<i32>, right: &Vec<i32>) {
    let mut total_sum: i32 = 0;

    for &left_val in left {
        let occurrences = right.iter().filter(|&&x| x == left_val).count() as i32;

        total_sum += left_val * occurrences;
    }

    println!("Part 2 - Total Sum: {}", total_sum);
}

pub fn run() {
    match read_data_from_txt("src/assets/day1/distances.txt") {
        Ok((mut left, mut right)) => {
            part1(&mut left, &mut right);
            println!("");
            part2(&left, &right);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}