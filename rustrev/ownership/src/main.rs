use std::io;
fn main() {
    println!("Hello, world!");
    copy();
}
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
fn copy() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Enter valid Input");
    println!("{}", is_even(13));
    let number = 13;
    let copynumber = number; //nummber is copied ,  not moved
    println!("Original Number:{}", number);
    println!("Copy Number:{}", copynumber);

    let flag = true;
    let copyflag = flag;
    println!("Flag:{}", flag);
    println!("copyflag:{}", copyflag);
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .expect("Enter valid input");
    println!("Number entered:{}", numbers)
}
