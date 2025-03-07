/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/

fn main() {
    /*
    ============================
    Scalar Types
    ============================

    Scalar types represent a single value. Rust has four primary scalar types:

    1. Integer Types:
        - Signed integers (can be positive or negative): i8, i16, i32, i64, i128, isize
        - Unsigned integers (only positive values): u8, u16, u32, u64, u128, usize

    Example:
    */
    let signed_integer: i32 = -42; // 32-bit signed integer
    let unsigned_integer: u32 = 42; // 32-bit unsigned integer

    /*
    2. Floating-Point Types:
        - f32: 32-bit floating-point number
        - f64: 64-bit floating-point number (default for floats)
    */
    let float_32: f32 = 3.14;
    let float_64: f64 = 2.71828;

    /*
    3. Boolean Type:
        - bool: Represents true or false values
    */
    let is_rust_hard: bool = false;

    /*
    4. Character Type:
        - char: Represents a single Unicode scalar value (4 bytes)
    */
    let letter: char = 'A';

    // Print scalar values
    println!("Signed Integer: {}", signed_integer);
    println!("Unsigned Integer: {}", unsigned_integer);
    println!("32 bit float: {}", float_32);
    println!("64 bit float: {}", float_64);
    println!("Is Rust Hard? {}", is_rust_hard);
    println!("Character: {}", letter);

    /*
    ============================
    Compound Types
    ============================

    Compound types can group multiple values into one type. Rust has two primitive compound types: arrays and tuples.

    1. Arrays:
        - Fixed-size collection of elements of the same type.
        - Syntax: [Type; length]
    */
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 integers
    println!("Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits array: {:?}", fruits);

    // Access array elements
    for (i, fruit) in fruits.iter().enumerate() {
        println!("Fruits Array element {}: {}", i + 1, fruit);
    }

    /*
    2. Tuples:
        - Fixed-size collection of elements of different types.
        - Syntax: (Type1, Type2, ...)
    */
    let person: (&str, i32, bool) = ("Alice", 30, true);
    let person2 = ("Hanif", 29, true);
    let person3: (String, i32, bool) = ("Ringlu".to_string(), 30, true);
    let mix_tuple = ("Apple", 23, false, [1, 2, 3, 4, 5]);

    println!("Person Tuple: {:?}", person);
    println!("Person 2 Tuple: {:?}", person2);
    println!("Person 3 Tuple: {:?}", person3);
    println!("Mixed Tuple: {:?}", mix_tuple);

    /*
    ============================
    Slices
    ============================

    Slices are references to a contiguous sequence of elements in a collection.
    They allow borrowing a portion of an array or string without copying the data.

    Syntax: &[Type]
    */
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Tiger", "Elephant"];
    println!("Animal slice: {:?}", animal_slices);

    /*
    ============================
    Strings and String Slices
    ============================

    - String: A growable, mutable, owned string type stored on the heap.
    - &str: A string slice, a reference to a section of a string.
    */
    let mut speech: String = String::from("Good ");
    println!("Someone Says: {}", speech);

    speech.push_str("Morning!");
    println!("Someone Says: {}", speech);

    let string: String = String::from("Good Morning!");
    let slice: &str = &string[0..5]; // String slice referencing the first 5 characters
    println!("Someone Says (slice): {}", slice);
}
