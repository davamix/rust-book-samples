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
    println!("The value of y is: {y}");

    let first_value = tup2.0;
    let second_value = tup2.1;
    
    println!("first_value: {first_value}");
    // println!("third_value: {tup2.2}"); // Not supported
    println!("third_value: {0}", tup2.2);

    let mut x: (i32, i32) = (1,2);
    x.0 = 0;
    x.1 += 5;

    println!("x value: {0}, {1}", x.0, x.1);
}
