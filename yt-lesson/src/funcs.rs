use std::i32;

// entrypoint
fn main() {
    hello_world();
    tell_height(178);

    human_id("brett", 32, 2.2);
    let _X: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("x is {}", _X);
    println!("fn add: {}", add(_X, 5));
    let y: i32 = add(_X, 5);
    println!("y is {}", y);
    println!("bmi is {}", bmi(2.00, 88.0))
}

fn hello_world() {
    println!("hello rust!");
}

fn tell_height(height: u32) {
    println!("my height is  {} cm", height);
}

fn human_id(name: &str, age: usize, height: f32) {
    println!(
        "hi i am {}, i am {} yrs young, and am {}m tall",
        name, age, height
    );
}
// outside of a function scope should be UPPER_SNAKE_CASE and const
// expressions versus statements
// expression: anything that returns a value
// statement: anything that does not return a value
// expressions
//
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn bmi(height: f32, weight: f32) -> f64 {
    (weight / (height * height)).into()
}
