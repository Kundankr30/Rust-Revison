use std::io;
fn main() {
    println!("Hello, world!");
    println!("Guess the number!");
    println!("Please input your guess.");
    let name = String::from("Kundan");
    println!("{}", name);
    let mut guess = String::new();
    //Take User and handle any error
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    let mut x = 100;
    println!("x:{}", x);
    x = 40; //Not Allowed
    println!("x: {}", x);
    for i in 1..=15 {
        println!("{}", i);
    }
    //for i in (1..=15).rev() {
    //    println!("{}", i);
    // }
    fn intadd(a: i32, b: i32) -> i32 {
        return a + b;
    }
    println!("{}", intadd(4, 5));
    fn floatadd(a: f64, b: f64) -> f64 {
        return a + b;
    }
    println!("{}", floatadd(3.45, 7.89))
}
