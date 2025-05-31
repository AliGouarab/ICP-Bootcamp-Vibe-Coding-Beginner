// Example 1: String ownership
fn example1() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Clone to preserve s1

    println!("{}, world!", s1); // Now s1 is still valid
}

// Example 2: Function ownership
fn example2() {
    let s = String::from("hello");
    takes_ownership(&s); // Borrow instead of move

    println!("After function call: {}", s); // Now s is still valid
}

fn takes_ownership(some_string: &String) {
    println!("Inside function: {}", some_string);
}

// Example 3: Vector ownership
fn example3() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v { // Iterate by reference to avoid consuming `v`
        println!("{}", i);
    }

    let sum: i32 = v.iter().sum(); // `v` is still valid
    println!("Sum: {}", sum);
}

fn main() {
    println!("Running Example 1:");
    example1();

    println!("\nRunning Example 2:");
    example2();

    println!("\nRunning Example 3:");
    example3();
}
