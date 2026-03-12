use std::io;

fn main() {
    println!("Hello, world!");
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    //Take User and handle any error
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    let mut x = 100;
    x = 40; //Not Allowed
    println!("x: {}", x);
}
