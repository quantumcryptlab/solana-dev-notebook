use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("Please guess a number between 1 and 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("\nEnter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) if num < 1 || num > 100 => {
                println!("Please enter a number between 1 and 100!");
                continue;
            }
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\nCongratulations, you win!");
                println!("You guessed the number {} in {} attempts!", secret_number, attempts);
                break;
            }
        }
    }
}
