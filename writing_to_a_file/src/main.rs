use std::fs::File;
use std::io::{self,Write};

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    println!("Input some text to save in a file");
    let mut text=String::new();
    io::stdin()
               .read_line(&mut text)
               .expect("Failed to read line");
    file.write_all(text.as_bytes())?;
    Ok(())
}
