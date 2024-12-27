use std::collections::HashMap;

fn main() {
    // collection type - hashmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 19);
    scores.insert(String::from("Green"), 11);

    let team_name: String = String::from("Green");
    let score: i32 = scores.get("Blue").copied().unwrap_or(0);
    // .copied converts Option<&T> to Option<T>
    // unwrap_or(default): Provides a default value for None
    let score2: i32 = scores.get(&team_name).copied().unwrap_or(0);
    println!("s {score} s2 {score2}");
    // &str: String Slice
    // Definition: A reference to a part of a string (could be the whole string or a substring). It is the most lightweight and commonly used string type in Rust.
    // Use Cases:
    // When working with string literals (&'static str).
    // When you only need to view a string as an immutable slice, regardless of whether it comes from a String or another &str.
    // It is preferred in function signatures for flexibility, as both String and &String can easily be converted to &str.
    for (key, value) in &scores {
        println!("{key} : {value}");
    }
}
