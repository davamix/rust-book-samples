// https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Write your number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read the number");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your number is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
