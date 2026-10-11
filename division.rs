use std::io;

fn main(){
    let mut input = String::new();
    println!("Please enter the first integer");
    io::stdin().read_line(&mut input).unwrap();
    let firstInteger: i32 = input.trim().parse().unwrap();
    input.clear();
    println!("Please enter the second integer");
    io::stdin().read_line(&mut input).unwrap();
    let secondInteger: i32 = input.trim().parse().unwrap();
    input.clear();
    let div =  firstInteger as f64 / secondInteger as f64;
    println!("The division of {firstInteger} by {secondInteger} is equal to {div}");
}
