/*
  Simple guessing game

  Don't call me a retard, I'm just a beginner lol ☻

  Feel free to work on it
*/

use std::cmp::Ordering;
use std::io;


use rand::Rng;

fn main() {
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=5);

    loop {
        println!("Please input ur guess.\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // let guess: u32 = guess.trim().parse().expect("please type a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
        }
    }

}