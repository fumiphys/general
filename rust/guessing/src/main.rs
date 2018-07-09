extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("secret number: {}", secret_number);
    println!("guess number: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read number");
    println!("guessed number: {}", guess);
    let guess: u32 = guess.trim().parse().expect("failed to parse");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
        Ordering::Equal => println!("equal"),
    }
}
