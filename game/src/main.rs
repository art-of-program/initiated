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

        let guess: u32 = guess.trim().parse().expect("Please tye a number");

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