/*
  Simple guessing game

  Don't call me a retard, I'm just a beginner lol ☻

  Feel free to work on it ⍫⍫
*/

use std::cmp::Ordering;
use std::io;


use rand::{Rng, thread_rng};

fn main() {
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=10);

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

        let probabilty = 10.0 / 10.0;

         println!("You guessed: {guess}\n");
         println!("secret number: {secret_number}\n");

         println!("Probability: {probabilty}");

        // let rng = thread_rng().gen_bool(0.5);

        // println!("\n{rng}\n");

        // match guess.cmp(&secret_number) {
        // Ordering::Less => println!("Too small!"),
        // Ordering::Greater => println!("Too big!"),
        // Ordering::Equal => {
        //     println!("You win!");
        //     break;
        // },
        // }
    }

}