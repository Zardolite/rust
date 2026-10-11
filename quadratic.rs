use std::io;

fn main(){
    let mut input = String::new();
    println!("What is the value of a?");
    io::stdin().read_line(&mut input).unwrap();
    let a: f32 = input.trim().parse().unwrap();
    if a == 0.0{
        println!("a cannot be equal to 0.");
        return;
    }
    input.clear();
    println!("What is the value of b?");
    io::stdin().read_line(&mut input).unwrap();
    let b: f32 = input.trim().parse().unwrap();
    input.clear();
    println!("What is the value of c?");
    io::stdin().read_line(&mut input).unwrap();
    let c: f32 = input.trim().parse().unwrap();
    input.clear();
    let delta: f32 = b.powf(2.0) - 4.0*a*c;
    let x1 = (-b)/(2.0*a);
    let xp = (-delta).sqrt()/(2.0*a);
    let x2 = (-b + delta.sqrt())/(2.0*a);
    let y2 = (-b - delta.sqrt())/(2.0*a);
    if delta < 0.0 {
        println!("This polynomial has two imaginary roots which are equal to: {x1} + i{xp} and {x1} - i{xp} ");
    }
    else if delta == 0.0{
        println!("this polynomial has one real root which is equal to: {}", x1);
    }
    else{
        println!("this polynomial has two real roots which are equal to: {x2} and {y2}");
    }
}
