use std::io;

fn main(){
    let mut input = String::new();
    println!("Enter a non-negative integer");
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut factorial: i32 = 1;
    for i in 1..=n{
        factorial *= i;
    }
    println!("Our result is {factorial}");
}
