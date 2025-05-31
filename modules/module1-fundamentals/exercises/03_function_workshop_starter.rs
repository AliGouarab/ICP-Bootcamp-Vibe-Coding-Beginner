// 1. Define a function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 2. Define a function that calculates the area of a rectangle
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

// 3. Define a function that checks if a number is prime
fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=((number as f64).sqrt() as u32) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

// 4. Define a function that converts Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    // Call the addition function with different values and print the results
    let sum1 = add(10, 20);
    let sum2 = add(-5, 15);

    // Calculate and print the area of rectangles with different dimensions
    let area1 = calculate_rectangle_area(4.5, 7.2);
    let area2 = calculate_rectangle_area(10.0, 3.0);

    // Test your prime number checker with several numbers
    let prime_check1 = is_prime(17);
    let prime_check2 = is_prime(18);

    // Convert and print some temperatures from Fahrenheit to Celsius
    let celsius1 = fahrenheit_to_celsius(32.0);
    let celsius2 = fahrenheit_to_celsius(100.0);

    // Print all results with appropriate labels
    println!("Sum of 10 and 20 is: {}", sum1);
    println!("Sum of -5 and 15 is: {}", sum2);

    println!("Area of rectangle with width 4.5 and height 7.2 is: {:.2} square units", area1);
    println!("Area of rectangle with width 10.0 and height 3.0 is: {:.2} square units", area2);

    println!("Is 17 a prime number? {}", prime_check1);
    println!("Is 18 a prime number? {}", prime_check2);

    println!("32°F is equivalent to {:.2}°C", celsius1);
    println!("100°F is equivalent to {:.2}°C", celsius2);
}
