use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let path = "2_input.txt";
    let nums = file_to_array(path)?;

    // print_matrix(&nums);

    let wrong_rows = count_safe_rows(&nums);
    //let wrong_rows = count_valid_rows(nums);

    print!("{}", wrong_rows);

    Ok(())
}
fn is_valid_row(row: &[i32]) -> bool {
    if row.len() < 2 {
        return true;
    }

    let mut ascending = true;
    let mut descending = true;

    for i in 0..row.len() - 1 {
        let diff = row[i + 1] - row[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff < 0 {
            ascending = false;
        }
        if diff > 0 {
            descending = false;
        }
    }

    ascending || descending
}

fn can_be_valid_with_one_removal(row: &[i32]) -> bool {
    for i in 0..row.len() {
        let new_row: Vec<i32> = row
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &v)| v)
            .collect();
        if is_valid_row(&new_row) {
            return true;
        }
    }
    false
}

fn count_safe_rows(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;

    for row in matrix {
        if is_valid_row(row) || can_be_valid_with_one_removal(row) {
            safe_count += 1;
        }
    }

    safe_count
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for &value in row {
            print!("{} ", value);
        }
        println!(); // Move to the next line after printing each row
    }
}

fn file_to_array(path: &str) -> io::Result<Vec<Vec<i32>>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        let row: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if !row.is_empty() {
            matrix.push(row);
        }
    }

    Ok(matrix)
}
