/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/

// Rust function offers hoisting opportunities.
// Hoisting: can call function anywhere in the code
// For example, in C/C++, we have to write or define the
// function above the main function. But it's not needed in Rust.

fn main() {
    greetings();
    birth_year(2012);
    release_date(12, "January");
    let x = multiply(32, 23);
    println!("The multiplication result is {}", x);

    // New function calls
    check_positive(-5);
    let mut num = 5;
    increment(&mut num);
    println!("Incremented value: {}", num);
    
    let name = format_name("fERris");
    println!("Formatted name: {}", name);
    
    let numbers = vec![1, 2, 3, 4, 5];
    let total = sum_numbers(&numbers);
    println!("Sum of numbers: {}", total);
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

// ============================
// Function Types and Examples
// ============================

// 1. Basic Function (No Parameters, No Return Value)
fn greetings() {
    println!("Hello world, I am Rust programming language!");
}

// 2. Function with Single Parameter
fn birth_year(year: i32) {
    println!("I was born in the year {}", year);
}

// 3. Function with Multiple Parameters
fn release_date(date: i32, month: &str) {
    println!("My first release date is {} {}", date, month);
}

// 4. Function with Return Value (Expression)
fn multiply(a: i32, b: i32) -> i32 {
    a * b // No semicolon = implicit return
}

// 5. Function with Conditional Logic and Boolean Return
fn check_positive(num: i32) {
    let is_positive = num > 0;
    println!("Is {} positive? {}", num, is_positive);
}

// 6. Function with Mutable Reference Parameter
fn increment(num: &mut i32) {
    *num += 1; // Modify the value being referenced
}

// 7. Function with String Manipulation and Return
fn format_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars.flat_map(|c| c.to_lowercase())).collect(),
    }
}

// 8. Function with Slice Parameter and Iterator
fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum() // Sum all elements in slice
}

// 9. Function with Error Handling (Result)
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
