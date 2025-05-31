use std::io;
use chrono::Local;

fn main() {
    // 1. Prompt the user for their name
    println!("Please enter your name:");

    // 2. Read the user's input
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    let name = name.trim(); // Remove trailing newline

    // 3. Print a personalized greeting
    println!("Hello, {}!", name);

    // BONUS: Print the current date
    let now = Local::now();
    println!("Today's date is: {}", now.format("%Y-%m-%d"));
}