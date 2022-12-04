use std::io;
use rand::Rng;

fn main () {
   
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut num_guess = String::new();
    
    io::stdin()
        .read_line(&mut num_guess)
        .expect("Failed to read number");

     println!("Your guess: {num_guess}");
    
    let num:i32 = num_guess.trim().parse().unwrap();
     println!("{}",num); 
  if num > secret_number {
        println!("Your guess is too high, the number was {secret_number}");
    }
    if num < secret_number {
        println!("Your guess is too low, the number was {secret_number}");
    }
   if num == secret_number {
       println!("Your guess is CORRECT")
   }
    

}
