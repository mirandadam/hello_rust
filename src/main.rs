extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 and 100!");

    let mut guess = String::new();
    // get a random number from 1 to 100. The upper bound is not inclusive.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number); //for debug purposes only

    loop {
        println!("Please input your guess.");

        guess.clear(); //clear previous result. read_line appends the new value to the old one.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
        // end loop.
    }
}
