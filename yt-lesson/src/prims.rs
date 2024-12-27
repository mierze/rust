// i8, i16, i32, i64, i128
// u8, u16, u32, u64, u128
fn main() {
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("max i32 {}", e);
    println!("max i64 {}", i);
    // f32, f64
    let f: f32 = 3.141596;
    println!("float {}", f);
    let b: bool = true;
    let c: char = 'c';
    println!("char is {}", c);
}
