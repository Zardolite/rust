use std::io;

fn main(){
    let mut input = String::new();
    println!("Please enter the length");
    io::stdin().read_line(&mut input).unwrap();
    let l: f64 = input.trim().parse().unwrap();
    input.clear();
    println!("Please enter the width");
    io::stdin().read_line(&mut input).unwrap();
    let w: f64 = input.trim().parse().unwrap();
    input.clear();
    println!("Please enter the height");
    io::stdin().read_line(&mut input).unwrap();
    let h: f64 = input.trim().parse().unwrap();
    input.clear();
    let V: f64 = l * w * h;
    println!("The Volume of {l} and {w} and {h} is equal to {V}");
}
