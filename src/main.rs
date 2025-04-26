use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    // a random number to guess     
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");
    // take user input
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    // typecast string to int
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    //Update user about there attempt
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
