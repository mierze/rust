fn main() {
    // vectors and hashmaps
    // common collections
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    let the_vec = vec![0, 1, 2, 2];
    // cool dynamic array that can grow or shrink as needed
    println!("v {:?}, the_vec {:?}", v, the_vec);
    let third = the_vec[3];
    println!("third {third}");

    let second = the_vec.get(1);
    match second {
        Some(second) => println!("{second}"),
        None => println!("no second"),
    }

    //
}
