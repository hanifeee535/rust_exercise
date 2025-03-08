/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/



fn main() {
    // Create an array of 100 elements, all initialized to 0
    // Arrays have fixed size and type in Rust
    let array_100_elements = [0; 100];
    
    // Create a simple array with 5 elements
    let a = [1, 2, 3, 4, 5];
    
    // Create a slice containing elements from index 1 (inclusive) to 4 (exclusive)
    // This will include elements 2, 3, and 4 from array 'a'
    let slice_of_a = &a[1..4];
    
    // Destructure a tuple into individual variables
    // Tuple pattern matching splits into name and age components
    let (name, age) = ("Furry Mcfurson", 3.5);
    
    // Create a tuple with multiple elements
    // Tuples can contain mixed types, but here all are integers
    let numbers = (1, 2, 3, 4, 5);
    
    // Debug print the array (requires :? format specifier for arrays)
    println!("{:?}", array_100_elements);
    
    // Debug print the slice (shows [2, 3, 4])
    println!(" {:?} ", slice_of_a);
    
    // Format string using destructured tuple values
    println!("{} is {} years old", name, age);
    
    // Access tuple elements by index (note: zero-based indexing)
    println!("The second element of numbers is: {}", numbers.1);
}