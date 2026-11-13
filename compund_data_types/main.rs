// arrays, tuples, slices, and strings (slice strings)

// Arrays
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);

    // let mix = [1, 2, "apple", true];
    // println!("Mix array: {:?}", mix);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits array 1st element: {}", fruits[0]);
    println!("Fruits array 2nd element: {}", fruits[1]);
    println!("Fruits array 3rd element: {}", fruits[2]);

    // tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5, 6]);
    println!("Mix tuple: {:?}", my_mix_tuple);

    // slices
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile", "Fox", "Hawk"];
    println!("Animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"Iron Mask".to_string(),
        &"Lord of the Rings".to_string(),
        &"Witcher".to_string(),
        &"Harry Potter".to_string(),
        &"The Alchemist".to_string(),
    ];
    println!("Book slices: {:?}", book_slices);

    // Strings Vs String Slices (&str)
    // Strings [ growable, mutable, owned string type ]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);

    // &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
    println!("String: {}", string);
}

// will cause a scope error

// fn print() {
//     println!("SLICE: {}", slice);
// }
