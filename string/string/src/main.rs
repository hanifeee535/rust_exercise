/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/
/*
In Rust, strings are a fundamental way to work with text. They come in different types, each with its own characteristics and use cases. 
different types of string:

1. String Slices (&str):
   - Immutable references to string data.
   - Often used for static strings or when we don’t need ownership.

2. String (Heap-Allocated, Growable):
   - A growable, mutable string stored on the heap.
   - Useful when we need to modify or build a string dynamically.

3. Literal Strings:
   - String literals are stored in the program’s binary with a 'static lifetime.
   - They are immutable and fixed in size.

4. Concatenation and Formatting:
   - Strings can be combined using the `+` operator or the `format!` macro.

5. Substrings and Iteration:
   - We can slice strings and iterate through characters.

Let’s see all these concepts in action:
*/


fn main() {
   //1. String with slices (&str)
   let slice : &str = "Hello world";
   println! ("string slice: {} ", slice);

   //2. string (heap-alocateds, growable)
   let mut heap_string = String :: from ("Hello");
   heap_string.push_str (", rust!");
   println! ("Heap-allocated string: {} ", heap_string);

   //3. literal strings
   let static_str : &'static str = "I am a static string!";
   println! ("Heap-allocated string = {} ", static_str);

   //4. concatenation and formating:
   let string1 = String::from ("Hello");
   let string2 = String :: from ("From rust program");
   let concatenated = format! (" {} {}", string1, string2);
   println! ("COncatenated string: {} ", concatenated);

   //5. Itterating over a string
   for c in heap_string.chars(){
    print! (" {} ", c);
   }
   println!();

   //6. getting a substring from a string
   let substr = &heap_string[0..5];
   println!("Substring = {}", substr);

   
}
