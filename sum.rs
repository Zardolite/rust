use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter the first integer");
    io::stdin().read_line(&mut input).unwrap();
    let firstInteger: i32 = input.trim().parse().unwrap();
    input.clear();
    println!("Please enter the second integer");
    io::stdin().read_line(&mut input).unwrap();
    let secondInteger: i32 = input.trim().parse().unwrap();
    input.clear();
    let sum: i32 = firstInteger + secondInteger;
    println!("The sum of {firstInteger} and {secondInteger} is {sum}")
}
