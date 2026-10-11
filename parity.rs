use std::io;

fn main(){
    let mut input = String::new();
    println!("Please enter an integer");
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();
    input.clear();
    let b = a%2;
    if b == 0{
        println!("the number {a} is an even number");
    }
    else{
        println!("the number {a} is an odd number");
    }
}
