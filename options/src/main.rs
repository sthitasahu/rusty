#![allow(unused)]

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
fn 

fn main() {
    // The return value of the function is an option
   // let result = divide(2.0, 3.0);
   let result=divide(5.0,0.0);

    // Pattern match to retrieve the value
    match result {
        // The division was valid
        Some(x) => println!("Result: {x}"),
        // The division was invalid
        None => println!("Cannot divide by 0"),
    }
}
