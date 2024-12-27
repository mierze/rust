use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let path = "3_input.txt";
    let contents = read_file_to_string(path)?;
    let result = process_instructions(&contents);
    println!("Sum of products: {}", result);
    Ok(())
}

// Function to read the entire file into a String
fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Function to process instructions and calculate the sum of valid mul operations
fn process_instructions(contents: &str) -> i32 {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\(\s*-?\d+\s*,\s*-?\d+\s*\))").unwrap();
    let mul_re = Regex::new(r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)").unwrap();

    let mut is_mul_enabled = true;
    let mut sum = 0;

    for cap in re.captures_iter(contents) {
        let instruction = &cap[0];

        if instruction == "do()" {
            is_mul_enabled = true;
        } else if instruction == "don't()" {
            is_mul_enabled = false;
        } else if let Some(mul_cap) = mul_re.captures(instruction) {
            if is_mul_enabled {
                let num1: i32 = mul_cap[1].parse().unwrap();
                let num2: i32 = mul_cap[2].parse().unwrap();
                sum += num1 * num2;
            }
        }
    }

    sum
}
//
// use regex::Regex;
// use std::fs::File;
// use std::io::{self, Read};
//
// fn main() -> io::Result<()> {
//     let path = "3_input.txt";
//     let contents = read_file_to_string(path)?;
//     let tuples = extract_tuples(&contents);
//     let result = sum_of_products(&tuples);
//
//     println!("Sum of products: {}", result);
//     Ok(())
// }
//
// // Function to read the entire file into a String
// fn read_file_to_string(path: &str) -> io::Result<String> {
//     let mut file = File::open(path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }
//
// // Function to extract tuples from the text using regex
// fn extract_tuples(contents: &str) -> Vec<(i32, i32)> {
//     let re = Regex::new(r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)").unwrap();
//     re.captures_iter(contents)
//         .filter_map(|cap| {
//             let num1 = cap[1].parse::<i32>().ok()?;
//             let num2 = cap[2].parse::<i32>().ok()?;
//             Some((num1, num2))
//         })
//         .collect()
// }
//
// // Function to compute the sum of the products of all tuples
// fn sum_of_products(tuples: &[(i32, i32)]) -> i32 {
//     tuples.iter().map(|(a, b)| a * b).sum()
// }
//
