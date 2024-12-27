fn main() {
    // arrays, tuples, slices, and strings (sliced)
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("num array {:?}", numbers);
    // let mix = [1, 2, "apple", true];
    // println!("mix {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("fruits {:?}", fruits);
    println!("1: {}", fruits[0]);
    println!("2: {}", fruits[1]);
    println!("3: {}", fruits[2]);

    // tuples
    let human = ("Alice", 30, false);
    // string
    // let humantyped: (String, i32, bool) = ("Alice", 30, false);
    // string slice fixed
    let humantyped: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("human {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("mixed {:?}", my_mix_tuple);

    // slices - contagious sequence of elements
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("number_slices {:?}", number_slices);
    let animal_slices: &[&str] = &["lion", "elephant", "rabbit"];
    println!("animal_slices {:?}", animal_slices);

    let books_slices: &[&String] = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"ZEN".to_string(),
    ];
    println!("books {:?}", books_slices);

    // strings are growable / mutable owned
    // Strings [ growable, mutable, owned string type
    // by default immutable
    // ex.
    let mut stone_cold: String = String::from("Hell, ");
    println!("stone cold says : {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("stone cold says : {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string;
    let slice_short: &str = &string[0..5];
    println!("slice {}", slice_short);
}
