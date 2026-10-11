use std::io;

fn main(){
    let mut input = String::new();
    println!("Enter a first integer");
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();
    input.clear();
    println!("Enter a second integer");
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();
    input.clear();
    println!("The maximum between these two values is {}", a.max(b))
}
