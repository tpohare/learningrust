extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut number_of_guesses = 1;

    loop {
        println!("Please input your guess.");

        let mut guess_string = String::new();

        io::stdin().read_line(&mut guess_string)
            .expect("failed to read line");

        let guess: u32 = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if "quit" == guess_string.trim() {
                    println!("Quitting");
                    break;
                }
                println!("That's not a number");
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("{} is too small!", guess),
            Ordering::Greater => println!("{} is too big!", guess),
            Ordering::Equal   => {
                println!("You win in {} guesses! Quitting the app", number_of_guesses);
                break;
            },
        }
        
        number_of_guesses += 1;
    }
}