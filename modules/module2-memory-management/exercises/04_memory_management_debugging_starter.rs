// Problem 1: Fix ownership errors
fn problem1() {
    // 1.1: Fix the double-move error
    let data = vec![1, 2, 3];
    let x = data.clone(); // clone to preserve ownership
    let y = data; // data is moved here
    println!("{:?} {:?}", x, y);

    // 1.2: Fix the ownership issue with the function
    let name = String::from("Rust");
    print_data(&name); // pass by reference
    println!("My name is {}", name); // name is still valid
}

fn print_data(data: &str) {
    println!("Data: {}", data);
}

// Problem 2: Fix borrowing conflicts
fn problem2() {
    // 2.1: Fix the mutable/immutable borrow conflict
    let mut numbers = vec![1, 2, 3];
    numbers.push(4); // mutate before borrowing
    let first = &numbers[0]; // immutable borrow after mutation
    println!("First element is: {}", first);

    // 2.2: Fix the multiple mutable borrows
    let mut data = String::from("Hello");
    {
        let ref1 = &mut data;
        *ref1 = String::from("Hello, ");
    }
    {
        let ref2 = &mut data;
        *ref2 += "Rust!";
    }
    println!("Data: {}", data);
}

// Problem 3: Fix dangling references
fn problem3() {
    // 3.1: Fix the dangling reference returned by the function
    let result = get_string();
    println!("Result: {}", result);

    // 3.2: Fix the issue with references outliving the data
    let data = vec![1, 2, 3];
    let reference = &data; // data is still in scope
    println!("Reference: {:?}", reference);
}

fn get_string() -> String {
    let s = String::from("I am a dangling reference");
    s // return ownership instead of reference
}

// Problem 4: Fix lifetime problems
fn problem4() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("short");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest string: {}", result); // must print inside this scope
    }
}

// Add lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Problem 5: Optimize unnecessary cloning
fn problem5() {
    let original = String::from("Rust Programming");
    let len = calculate_length(&original); // pass by reference

    let names = vec![String::from("Alice"), String::from("Bob")];
    for name in &names {
        print_string(name); // pass by reference
    }

    println!("Original is still: {}", original);
    println!("Length was: {}", len);
    println!("Names: {:?}", names);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn print_string(s: &str) {
    println!("{}", s);
}

fn main() {
    println!("Uncomment and fix each problem section one by one.");
    println!("Once fixed, you can run each problem function from main.");

    // All problems fixed and called:
    problem1();
    println!();
    problem2();
    println!();
    problem3();
    println!();
    problem4();
    println!();
    problem5();
}
