use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, Read}; // Removed BufRead as it's not needed
use std::path::Path;
// Function to split input into two parts (rules and specs)
fn split_by_newline(s: String) -> (String, String) {
    // Split the string by double newline (`\n\n`)
    let parts: Vec<&str> = s.split("\n\n").collect();

    // If there are at least two parts, return the first two as a tuple
    if parts.len() >= 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        // If there aren't enough parts, return empty strings as a fallback
        ("".to_string(), "".to_string())
    }
}

// Function to parse the rules part
fn parse_rules(s: &str) -> Vec<(u32, u32)> {
    s.lines()
        .map(|line| {
            // For each line, split by '|' to get the range parts
            let parts: Vec<u32> = line
                .split('|')
                .map(|num| num.parse().expect("Invalid number in rule"))
                .collect();

            // Return the range as a tuple (start, end)
            (parts[0], parts[1])
        })
        .collect()
}
/// Function to parse the specs from a string, splitting by newlines first, then by commas for each row.
fn parse_specs(s: &str) -> Vec<Vec<u32>> {
    s.split('\n') // Split by newlines to get each row
        .filter_map(|line| {
            if !line.trim().is_empty() {
                Some(
                    line.split(',')
                        .filter_map(|num_str| num_str.trim().parse().ok()) // Parse each number and skip invalid ones
                        .collect::<Vec<u32>>(),
                )
            } else {
                None
            }
        })
        .collect()
}
/// Function to check that each row respects the rules.
fn validate_row(specs_row: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for (x, y) in rules {
        if let (Some(x_index), Some(y_index)) = (
            specs_row.iter().position(|&n| n == *x),
            specs_row.iter().position(|&n| n == *y),
        ) {
            if x_index > y_index {
                // If x appears after y in the row, the rule is violated
                return false;
            }
        }
    }
    true
}
// Function to read the input from file and process it
fn input_from_file() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    println!("Current working directory: {:?}", env::current_dir());

    // Read the content of the file
    let mut data = String::new();
    let mut f = File::open("./src/input1.txt").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    // Split the input into two parts (rules and specs)
    let (rules_str, specs_str) = split_by_newline(data);

    // Parse the rules and specs
    let parsed_rules = parse_rules(&rules_str);
    let parsed_specs = parse_specs(&specs_str);

    // Print parsed data (optional, for debugging)
    println!("Parsed rules: {:?}", parsed_rules);
    println!("Parsed specs: {:?}", parsed_specs);

    (parsed_rules, parsed_specs)
}

fn filter_specs(rules: Vec<(u32, u32)>, specs: Vec<u32>) -> Option<(Vec<u32>, u32)> {
    // Step 1: Validate the order of elements based on the rules
    for (x, y) in &rules {
        // Check if both x and y are in specs, and ensure x comes before y
        if let (Some(x_index), Some(y_index)) = (
            specs.iter().position(|&n| n == *x),
            specs.iter().position(|&n| n == *y),
        ) {
            if x_index > y_index {
                // If x appears after y in the list, the rules are violated
                return None;
            }
        }
    }

    // Step 2: If no rules are violated, proceed to find the valid ordering
    // We don't need to actually reorder the specs, just check if the rules are respected
    let mut result = specs.clone();

    // Step 3: Calculate the sum of centered elements (those not at the first or last position)
    let sum_centered: u32 = result
        .iter()
        .skip(1) // Skip the first element
        .take(result.len() - 2) // Skip the last element
        .sum();

    // Return the result along with the sum of centered elements
    Some((result, sum_centered))
}

/// Function to process the specs (as a 2D vector), validate the rules, and calculate the sum of centered elements.
fn process_specs(
    rules: Vec<(u32, u32)>,
    parsed_specs: Vec<Vec<u32>>,
) -> Option<(Vec<Vec<u32>>, u32)> {
    // Step 1: Validate each row against the rules
    for row in &parsed_specs {
        if !validate_row(row, &rules) {
            return None; // Return None if any row violates the rules
        }
    }

    // Step 2: Calculate the sum of centered elements (those not at the start or end of each row)
    let mut sum_centered = 0;
    for row in &parsed_specs {
        if row.len() > 2 {
            sum_centered += row
                .iter()
                .skip(1) // Skip the first element
                .take(row.len() - 2) // Skip the last element
                .sum::<u32>(); // Sum the centered elements
        }
    }

    // Step 3: Return the valid rows and the sum of centered elements
    Some((parsed_specs, sum_centered))
}

// --- Day 5: Print Queue ---
fn main() {
    let (rules, specs) = input_from_file();

    // Output the parsed data for verification
    println!("Parsed rules: {:?}", rules);
    println!("Parsed specs: {:?}", specs);
    match process_specs(rules, specs) {
        Some((valid_specs, sum)) => {
            println!("Valid ordering in each row: {:?}", valid_specs);
            println!("Sum of centered elements: {}", sum);
        }
        None => println!("No valid ordering exists."),
    }
}
