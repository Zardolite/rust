use std::{thread::sleep, time::Duration};

fn main(){
    for i in (1..=10).rev(){
        println!("{i}");
        sleep(Duration::from_secs(1));
    }
    println!("BOOM");
}
