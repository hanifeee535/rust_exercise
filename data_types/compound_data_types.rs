/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/


fn main (){


    /*
        arrays: Fixed size data collection of the same data types
     */
    let numbers : [i32; 5] = [1,2,3,4,5]; //array of 5 integers
    println! ("Array: {:?}", numbers); 

    let fruits : [&str; 3] = ["Apple", "Banana", "Orange"]; 
    println! ("Fruits array {:?}", fruits);
    println! ("Fruits Array 1st element: {} ", fruits[0]);
    println! ("Fruits Array 2nd element: {} ", fruits[1]);
    println! ("Fruits Array 3rd element: {} ", fruits[2]);





       /*
        tuples: contains heterogenious collection of elements of fixed size
     */
// Case 1: Explicitly Typed Tuple
    // The tuple `person` is explicitly annotated with types: (&str, i32, bool).
    // - &str: A string slice (reference to a string).
    // - i32: A 32-bit signed integer.
    // - bool: A boolean value (true or false).
    let person: (&str, i32, bool) = ("Alice", 30, true);
    println!("Person Tuple: {:?}", person);

    // Case 2: Implicitly Typed Tuple
    // The tuple `person2` has no explicit type annotation. Rust infers the types based on the values:
    // - "Hanif" is inferred as &str.
    // - 29 is inferred as i32.
    // - true is inferred as bool.
    let person2 = ("Hanif", 29, true);
    println!("Person 2 tuple: {:?}", person2);

    // Case 3: Tuple with `String` Type
    // The tuple `person3` is explicitly annotated with types: (String, i32, bool).
    // - String: An owned, growable string (not a reference).
    // - i32: A 32-bit signed integer.
    // - bool: A boolean value.
    // The value "Ringlu" is converted to a String using `.to_string()`.
    let person3: (String, i32, bool) = ("Ringlu".to_string(), 30, true);
    println!("Person Tuple: {:?}", person3);

    // Case 4: Mixed-Type Tuple
    // The tuple `mix_tuple` contains elements of different types:
    // - "Apple" is inferred as &str.
    // - 23 is inferred as i32.
    // - false is inferred as bool.
    // - [1, 2, 3, 4, 5] is inferred as [i32; 5] (an array of 5 integers).
    let mix_tuple = ("Apple", 23, false, [1, 2, 3, 4, 5]);
    println!("Mixed tuple: {:?}", mix_tuple);




    /*
    Slices: a reference to a contiguous sequence of elements in a collection.
    It allows to borrow a portion of an array or another collection without copying the data.
    Slices are denoted by the type `&[T]`, where `T` is the type of the elements.
    They are commonly used for working with parts of arrays or string
     */


    // Example 1: Slice of integers
    // - `number_slices` is a slice of type `&[i32]`, referencing a portion of an array of integers.
    // - The slice borrows the array `[1, 2, 3, 4, 5]` without owning it.
    let number_slices : &[i32] = &[1,2,3,4,5]; 
    println! ("Number slice : {:?}", number_slices);

    // Example 2: Slice of string slices
    // - `animal_slices` is a slice of type `&[&str]`, referencing a portion of an array of string slices.
    // - The slice borrows the array `["Lion", "Tiger", "Elephant"]` without owning it.
    let animal_slices : &[&str] = &["Lion", "Tiger", "Elephant"]; 
    println! ("Animal slice: {:?} ", animal_slices);

    



    
   /*
    String vs String Slices (&str):
    - `String`: A growable, mutable, owned string type stored on the heap.
    - `&str`: An immutable reference to a string slice, which can point to a part of a `String` or a string literal.
*/

// Example 1: Using `String`
// - `String` is an owned, growable, and mutable string type.
// - It is stored on the heap, allowing it to dynamically resize as needed.
let mut speach: String = String::from("Good ");
println!("Someone Says: {}", speach);

// Modifying the `String`:
// - The `push_str` method appends a string slice to the `String`.
// - This demonstrates the mutable nature of `String`.
speach.push_str("Morning!");
println!("Someone Says: {}", speach);

// Example 2: Using `&str` (String Slice)
// - `&str` is an immutable reference to a string slice.
// - It can point to a part of a `String` or a string literal.
let string: String = String::from("Good Morning!");

// Creating a string slice:
// - The slice `&string[0..5]` references the first 5 bytes of the `String`.
// - In this case, it points to "Good " (indices 0 to 4).
let slice: &str = &string[0..5];
println!("Someone Says: {}", slice);

}