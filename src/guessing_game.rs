/*
* Number Guessing Game
* |-------------------|
*
* Created by AhmadAEngineer
* ------21/10/2024---------
*
* Description:
* ____________
* This is the number guessing game, and it is fun to play in your spare time.
* As it name suggest it is guessing game, you will be asked for to enter your guess
* then it will compare that guess with the computer generated secret number.
* If you guessed the right number you will win!
* If not it will ask you for re-enter and also give you suggestion whether the number
* is large or small, it will help you find the right number.
*
* Features:
* _________
*  -Suggestions.
*  -Play Again.
*  -Number of attempts counter.
*
* Instructions:
* _____________
* 1. Clone the repo to your local computer.
* 2. Compile the "main.rs" file in your computer using "Rust" compiler (If you are
* not a developer/programmer simply download the "main.exe" file and run it in your computer)
*
* |------------ Happy Guessing ---------------|
 */

use std::io::stdin;
use std::cmp::Ordering;
use std::process;
use rand::{thread_rng, Rng};

const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 10;

// Reading input
fn reading_input (prompt: &str) -> u32 {
    println!("{prompt}");

    let mut user_guess = String::new();

    match stdin().read_line(&mut user_guess) {
        Ok(_) => { },
        Err(e) => {
            println!("Failed to read input {e}");
            process::exit(1);
        }
    }

    let user_guess = match user_guess.trim().parse::<u32>() {
        Ok(success) => success,
        Err(e) => {
            println!("Error: failed to convert input to number: {}", e);
            process::exit(1);
        }
    };

    user_guess
}

fn random_number() -> u32 {
    let secret_number: u32 = thread_rng().gen_range(MIN_NUMBER..=MAX_NUMBER);
    secret_number
}

fn main() {
    loop {
        println!("Number Guessing Game");
        let secret_number = random_number();
        let mut counter = 0;
        loop{
            counter += 1;
        let prompt = "Please enter your guess: ";
        match reading_input(prompt).cmp(&secret_number) {
            Ordering::Less => {
                println!("Too Small, attempts: {}", counter);
            }
            Ordering::Greater => {
                println!("Too Large, attempts: {}", counter);
            }
            Ordering::Equal => {
                println!("Congrats! You Win, attempts {}", counter);
                counter = 0;
                break;
            }
        }
    }
        let user_choice = reading_input("Want to play again? Press '1' OR '0' to close the game!");
        if user_choice == 1 {
        continue;
        }
        else {
            println!("Game closed successfully!");
            break;
        }
    }
    process::exit(0);
}
