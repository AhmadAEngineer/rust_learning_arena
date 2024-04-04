use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("!!! ********** ~~~ Guessing Game !!! ********** ~~~");

    let secret_number = rand::thread_rng().gen_range(1..10);

    loop {
        println!("Enter the number to guess = ");

        let mut user_guess: String = String::new();
        match std::io::stdin().read_line(&mut user_guess) {
            Err(_) => println!("Failed to read number! "),
            Ok(_) => println!("You have guessed {user_guess}")
        };


        let user_guess: u32 = match user_guess.trim().parse() {
            Err(_) => println!("something went wrong!"),
            Ok(number) => number
        };

        // comparing the numbers
        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("To Big!"),
            Ordering::Equal => {println!("You Win!!!");
            break}
        };

    }
}