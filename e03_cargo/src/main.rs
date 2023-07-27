use std::io;

fn main() {
    // Read user input
    println!("Enter your name:");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let name = input_text.trim();

    // Display the input
    println!("Hello, {}! Nice to meet you!", name);
}