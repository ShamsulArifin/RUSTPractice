// Primitive data types
// int. float, bool, char

// Integer
// Rust has signed (+ and -) and unsigned
// integer (only+) types of different sizes.
// i8, i16, i32, i64, i128: signed integers.
// u8, u16, u32, u64, u128: unsigned integers.

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("signed integer: {}", x);
    println!("unsigned integer: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775800;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    // floats
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // bool
    let is_snowing: bool = true;
    println!("Is is snowing?: {}", is_snowing);

    // char
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}
