use std::io;
use rand::prelude::*;


fn main() {
   println!("Welcome to guessing game");
   let guess_list=["banana","apple","mango","orange","grapes"];
   loop{

    
    let mut rng=thread_rng();
    let random_number=rng.gen_range(0..guess_list.len());
    let random_fruit=guess_list[random_number];
    println!("Random fruit :{}",random_fruit);

    println!("Plz enter your guess :");
    let mut input=String::new();
    let mut fruit_name=String::new();

      match io::stdin().read_line(&mut input){
       Ok(_)=>{
         fruit_name=input.trim().to_lowercase().to_string();
         println!("Selected fruit is :{}",fruit_name);

         if !guess_list.contains(&fruit_name.as_str()){
            println!("Invakid fruit!Plz enter a valid fruit.");
            continue;

         }
       }
       Err(error)=>{
         println!("Error:{}",error);
       }
    }


    match check(&fruit_name,&random_fruit){
        true => {
            println!("You guessed it!");
            break;
        }
        false=>{
            println!("Try again!");
        }
    }
   }


}
fn check(guess:&str,actual:&str)->bool{
    guess==actual
}