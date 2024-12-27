use glam::IVec2;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let path = "4_input.txt";
    let matrix = file_to_str(path)?;
    let count = count_crosses(&matrix);
    print!("{}", count.to_string());

    Ok(())
}

fn file_to_str(path: &str) -> io::Result<String> {
    let input = File::open(path)?; // Open the file
    let mut reader = BufReader::new(input); // Wrap it in a BufReader
    let mut content = String::new(); // Create a String to hold the content
    reader.read_to_string(&mut content)?; // Read the file's content into the String
    Ok(content) // Return the content
}

fn count_crosses(input: &str) -> usize {
    const DIRECTIONS: [[IVec2; 2]; 4] = [
        [IVec2::new(-1, -1), IVec2::new(1, 1)],
        [IVec2::new(-1, 1), IVec2::new(1, -1)],
        [IVec2::new(1, 1), IVec2::new(-1, -1)],
        [IVec2::new(1, -1), IVec2::new(-1, 1)],
    ];

    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            // for each line map over chars into ivec2
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>();
    let mas = ['M', 'S'];
    // now is all flat
    let result: usize = positions
        .iter()
        .filter(|(_position, value)| **value == 'A')
        .filter(|(position, _value)| {
            DIRECTIONS
                .iter()
                .map(|ms_positions| {
                    ms_positions
                        .iter()
                        .map(|pos| positions.get(&(*position + pos)))
                        .enumerate()
                        .all(|(index, value)| mas.get(index) == value)
                })
                .filter(|b| *b)
                .count()
                == 2
        })
        .count();
    result
}
