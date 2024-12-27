use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let path = "1_input.txt";
    let (mut nums1, mut nums2) = file_to_arrays(path)?;

    nums1.sort();
    nums2.sort();

    let distance = distance_sum(&nums1, &nums2);
    let similarity = similarity_score(&nums1, &nums2);
    println!("distance: {}, similarity: {}", distance, similarity);

    Ok(())
}

fn file_to_arrays(path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        if let Some((n1, n2)) = line.split_once(' ') {
            match n1.trim().parse::<i32>() {
                Ok(n) => nums1.push(n),
                Err(e) => eprintln!("parse err for '{}': {}", n1, e),
            }
            match n2.trim().parse::<i32>() {
                Ok(n) => nums2.push(n),
                Err(e) => eprintln!("parse err for '{}': {}", n2, e),
            }
        }
    }

    Ok((nums1, nums2))
}

fn distance_sum(nums1: &[i32], nums2: &[i32]) -> i32 {
    nums1
        .iter()
        .zip(nums2.iter())
        .map(|(&n1, &n2)| (n1 - n2).abs())
        .sum()
}

fn similarity_score(nums1: &[i32], nums2: &[i32]) -> i32 {
    let mut nums2_count = HashMap::new();

    for &num in nums2 {
        *nums2_count.entry(num).or_insert(0) += 1;
    }

    let mut total_score = 0;

    for &num in nums1 {
        if let Some(&count) = nums2_count.get(&num) {
            total_score += num * count;
        }
    }

    total_score
}
