use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {guess}")

    let guess: u32 = guess.trim().parse().expect("Please type a number!") 

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater=> println!("Too Big!"),
        Ordering::Equal=> println!("You Win!")
    }
}
