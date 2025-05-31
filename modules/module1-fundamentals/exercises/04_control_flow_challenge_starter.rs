use std::io;

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    for i in 1..=20 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    let mut running = true;
    
    while running {
        println!("\nChoose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        // Get user choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        if choice == 5 {
            running = false;
            break;
        }
        
        // Get first number
        println!("Enter the first number:");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read input");
        let num1: f64 = match num1.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number input.");
                continue;
            }
        };
        
        // Get second number
        println!("Enter the second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read input");
        let num2: f64 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number input.");
                continue;
            }
        };
        
        // Perform operation
        match choice {
            1 => println!("Result: {}", num1 + num2),
            2 => println!("Result: {}", num1 - num2),
            3 => println!("Result: {}", num1 * num2),
            4 => {
                if num2 == 0.0 {
                    println!("Error: Cannot divide by zero.");
                } else {
                    println!("Result: {}", num1 / num2);
                }
            }
            _ => println!("Invalid option. Please try again."),
        }
        
        // Ask if user wants to continue
        println!("Do you want to perform another calculation? (y/n):");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read input");
        let again = again.trim().to_lowercase();
        
        if again != "y" {
            running = false;
        }
    }
    
    println!("Thank you for using the calculator!");
}
