fn main() {
    let mut x = 5;
    let _r = &mut x;
    *_r += 1;
    *_r += -3;

    println!("value of x is: {}", x);
}
