fn main() {
    // collection types - utf encoding
    let mut new_str: String = String::from("Hello growable / mutable");
    new_str.push_str("heool");
    print!("{new_str}");

    let mut s: String = "whatever".to_string();
    s.push_str("heool");
    s.push('n');
    print!("\n{s}\n");

    // if you want to combine strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note: s1 has been moved here
    println!("s1{}", s3);

    let test1: String = String::from("test1");
    let test2: String = "test2".to_string();
    let full_msg: String = format!("{test1} {test2}");
    println!("{full_msg}");
}
