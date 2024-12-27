// if / else
fn main() {
    let age: u16 = 18;

    // if age >= 18 {
    //     println!("you are an adult");
    // } else {
    //     println!("you can't drive a car!");
    // }
    //
    // multiple conditions with else if
    let number = 6;
    if number % 4 == 0 {
        println!("number divisible by 4");
    } else if number % 3 == 0 {
        println!("number divisible by 3");
    } else if number % 2 == 0 {
        println!("number divisible by 2");
    } else {
        println!("not");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {number}"); // 5
}
