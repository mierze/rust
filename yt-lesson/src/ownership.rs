fn main() {
    // ownership
    // [stopping/resuming the program
    // OWNERSHIP introduced by rust to solve memlry safety issues and high performance at the
    // same time.
    // what is ownership?
    // every value has a single owner [every variable has one value, and it is its sole owner]
    // RULES
    // 1- each value in rust has a variable that's its owner.
    let s1 = String::from("RUST");
    let len = cal_len(&s1);
    println!("len of '{}' is {}", s1, len);
    // 2- there can be only one owner at a time.
    let s = String::from("RUST");
    let s2 = s;
    // println!("s {}", s);
    println!("s2 {}", s2);
    // 3- when the owner goes ou of scope, the value will be dropped.
    let s3 = String::from("rust");
    let newlen = cal_len(&s3);
    println!("len of {}", newlen);
}

fn cal_len(s: &String) -> usize {
    s.len()
}
