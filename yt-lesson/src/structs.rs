fn main() {
    // structs
    // tuple
    let rect: (i32, i32) = (100, 200);

    // struct
    struct Book {
        title: String,
        author: String,
        pages: i32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        age: u64,
    }

    let mut u: User = User {
        active: true,
        username: "hey".to_string(),
        age: 15,
    };
    u.username = String::from("email1");
    println!("u{}", u.username);

    // return struct from function
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            age: 1,
        }
    }
    // Source Type:
    //
    // .to_string() works on any type that implements ToString (via Display).
    // String::from is specifically designed to convert &str into String.
    // Performance:
    //
    // For string slices (&str), String::from is slightly more direct and explicit than .to_string().
    // .to_string() might have a small overhead because it goes through the Display implementation.
    // Clarity:
    //
    // Use String::from when explicitly converting from &str to String.
    // Use .to_string() when converting other types (e.g., integers, floats, or complex types with Display).
    // create instances from other instances
    let u2: User = User {
        username: String::from("anotherm@c.com"),
        ..u
    };
    // all same but the username
    // println!("u2 {:?}", u2.into());

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
}
