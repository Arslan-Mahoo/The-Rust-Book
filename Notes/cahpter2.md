-This chapter is about making a number Guessing game 
Chapter introduces 
1. Random number generation 
I used use rand::prelude::*; at the top of file to use rand lib
2. I used use std::io; at the top of file
3. comparison using Ordering
I used use std::cmp::Ordering; at the top of file 
4. How we get input from user?
a. create a mutable variable with let and mut keyword i.e. let mut guess = String::new(); 
b. io::stdin().read_line({put address of created variable here+mut} , &mut guess).expect("Couldn't read line)
let guess: i32 = guess.trim().parse().expect("Please Enter a number ") changes guess: String to guess: i32
5. How to generate a random number 
a. create a rng by let mut rng = rnd::rng();
b. let secret_number = rng.random_range(1..=101); [Gives a number between 1 and 101]


6. Comparison 
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
}

7. put all game in loop but keep sexret_number generation out of loop 
secret number generation
loop {
 all game logic here 
}

Ordering::Equal => {
    logic
    break; This line is here for breaking game loop
}
