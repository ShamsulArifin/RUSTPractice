// Ownership, Borrowing and References

// Ownership
// ---------

// Each value in Rust has a variable that's its owner.

// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("Length of {} is: {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// There can be only one owner at a time

// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1;

//     println!("{}", s2);
// }

// When the owner goes out of scope, the value will be dropped

fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);
}

// s1 goes out of scope and its value will be dropped

fn calculate_length(s: &String) -> usize {
    s.len()
}
