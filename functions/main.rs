fn main() {
    println!("Hello World!");
    hello_world();
    tell_height(167);
    human_id("Sam", 30);

    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };

    println!("Result is: {}", _x);

    let y: i32 = add(3, 20);
    println!("Value of y is: {}", y);
    println!("Value from function 'add' is: {}.", add(4, 6));

    //calling the BMI function
    let bmi: f64 = calculate_bmi(70.7, 1.67);
    println!("My BMI score is: {}", bmi);
}

// hoisting - can call function anywhere in your code
fn hello_world() {
    println!("Hello World! from the function");
}

// you can insert input values
fn tell_height(height: i32) {
    println!("My height is {}", height);
}

// you can insert more than one value
fn human_id(name: &str, age: i32) {
    println!("Hello, my name is {}. I am {} years old", name, age);
}

// functions returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// final example
// BMI = height(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height: f64) -> f64 {
    weight_kg / (height * height)
}

// Expressions and Statements

// Expression: Anything that returns a value

// Statement: Anything that does not return a value
// Almost all statements in Rust end with ;
// let y = let x = 10;
// 1 variable declarations: let x = 5;
// 2 function definitions: fn foo() {}
// 3 control flow statements: if condition { /* code */ } else { /* code */}, while condition {/* code */}, etc.

// Expression
// 5
// true & false
// add(3,4)
// if condition
// ({code})
