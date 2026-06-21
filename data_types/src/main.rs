#![allow(unused)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number");

    // characters
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuples
    let tup:(i32, f64, u8) = (500, 6.4, 1);

    // tuples + pattern matching
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The value of y is: {y}")
}
