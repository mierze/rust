use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let path = "4_input.txt";
    let matrix = file_to_array(path)?;
    let count = find_xmas(matrix);
    print!("{}", count);

    Ok(())
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for &value in row {
            print!("{} ", value);
        }
        println!(); // Move to the next line after printing each row
    }
}

fn file_to_array(path: &str) -> io::Result<Vec<Vec<char>>> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let matrix = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    Ok(matrix)
}
fn search_in_direction(
    matrix: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    dx: isize,
    dy: isize,
) -> bool {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;
    let mut x = start_row as isize;
    let mut y = start_col as isize;
    let word = ['X', 'M', 'A', 'S'];

    for &ch in &word {
        if x < 0 || y < 0 || x >= rows || y >= cols || matrix[x as usize][y as usize] != ch {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

fn find_xmas(matrix: Vec<Vec<char>>) -> usize {
    let directions = [
        (1, 0),   // Right
        (-1, 0),  // Left
        (0, 1),   // Down
        (0, -1),  // Up
        (1, 1),   // Diagonal Down-Right
        (-1, -1), // Diagonal Up-Left
        (1, -1),  // Diagonal Down-Left
        (-1, 1),  // Diagonal Up-Right
    ];

    let mut count = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                for &(dx, dy) in &directions {
                    if search_in_direction(&matrix, i, j, dx, dy) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
