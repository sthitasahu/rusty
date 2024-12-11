use std::fs;
fn main() {
    println!("THis is a basic example on how to read contents of a file in rust using the standard file module in RUST");
    let filepath="hello.txt";
    let contents_of_the_file=fs::read_to_string(filepath);
    match contents_of_the_file{
        Ok(content)=>{
            println!("The contents of the file:{} are as follows:\n{}",filepath,content);
        }
        Err(error)=>{
            println!("Cannot read from this file because:{}",error);
        }
    }
    

}
