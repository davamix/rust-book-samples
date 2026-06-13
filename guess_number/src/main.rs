use std::io;

fn main() {
    println!("Guess the number");

    println!("Write your number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read the number");

    println!("Your number is {guess}");
}
