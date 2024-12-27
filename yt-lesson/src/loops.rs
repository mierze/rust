fn main() {
    // looping mechanisms
    // Loop

    // will continue until you tell stop
    // loop { // infinite
    //     println!("hello world");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("res {result}");

    // loop labels
    let mut count = 0;

    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    //
    //
    // While
    let mut c = 0;
    while c < 10 {
        print!("c");
        c += 1;
    }
    println!("\n");

    //
    // For loop
    let a = [1, 2, 3, 4, 5, 6, 7];
    for i in a {
        println!("{i}");
    }
}
