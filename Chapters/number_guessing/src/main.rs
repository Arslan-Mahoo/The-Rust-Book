use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("This is start of number guessing game ");
    let mut rng = rand::rng();
    let secret_number: i32 = rng.random_range(1..=101);
    loop {
    println!("Please Enter your guess here :");
    let mut guess = String::new();
    
    io::stdin()
    .read_line(&mut guess).expect("Couldn't read line");
    println!("You guessed {}", guess);
    let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    match guess.cmp(&secret_number){
        Ordering::Less => println!("too low"),
        Ordering::Greater => println!("too high"),
        Ordering::Equal => {println!("You win!!!!");
        break;
    }
        
    }

}
}
