use std::io;

fn main(){
    let mut input = String::new();
    println!("Give me an integer");
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();
    println!("x = {a} \t\t x square = {}", a*a)
}
