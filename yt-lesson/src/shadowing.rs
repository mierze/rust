// shadowing
fn main() {
    //
    let x = 4; // 1st x is shadowed by second x
    let x = x + 1;
    println!("x {} ", x); // 5
    {
        let x = x * 2;
        println!("value is {}", x); // 10
    }
    println!("x outside is {}", x); // does not know inner scope, back to 5

    // shadowing is not the same as marking a variable as mut
    // cannot use mut and this approach
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces {}", spaces);
    //
}

