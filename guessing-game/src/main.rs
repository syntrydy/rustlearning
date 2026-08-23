use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io,
};

fn main() {
    println!("Welcome to the guessing game");
    let secret = rand::random_range(1..=100);
    loop {
        let choice = get_user_choice();
        let choice = match choice.trim().parse::<u8>() {
            Ok(value) => value,
            Err(message) => {
                println!(
                    "The value should be an integer between 1 and 100.: Detail {}",
                    message
                );
                continue;
            }
        };
        match choice.cmp(&secret) {
            Less => {
                println!("Too small!")
            }
            Greater => {
                println!("Too big!")
            }
            Equal => {
                println!("Bravo, you have won.");
                break;
            }
        }
    }
}

fn get_user_choice() -> String {
    println!("Enter a number between 0 and 100 : ");
    let mut user_choice = String::new();
    let _ = io::stdin()
        .read_line(&mut user_choice)
        .expect("Error reading the user choice");
    user_choice
}
