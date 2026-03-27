use std::io;
fn check_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
fn main() {
    println!("Welcome to Even Checker");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Enter valid Input");
    let n: i32 = n.trim().parse().expect("Please enter a number");
    println!("{}", check_even(n));
    //loops
    for i in 1..6 {
        println!("{}", i);
    }
}
