use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number=rand::thread_rng().gen_range(1..=100);
    println!("Your Secret Number is:{secret_number}");
    println!("Guess the number!");
   loop{
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    

    let guess:u32=guess.trim().parse().expect("Please enter a number !!!");

    println!("You guessed: {}", guess);


    match guess.cmp(&secret_number){
        Ordering::Less =>println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
    }
    }
  }
}