mod array;

use std::io::{self, Write};
fn main() {
    println!("Write your name");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // Remove any trailing newline characters
    println!("Hello,{name}!")

} 


    