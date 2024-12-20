use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize,Deserialize,Debug)]
struct Person{
    name:String,
    age:u8,
    email:String
}
fn main() {
    println!("Hello, world!");
    let person=Person{
        name:String::from("Sthita"),
        age:20,
        email:String::from("sthitasahu011@gmail.com")
    };
    let  serialized=serde_json::to_string(&person).unwrap();
    println!("Serialized : {}",serialized);
}
