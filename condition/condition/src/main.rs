/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/

// ============================
// If, Else If, Else Structure in Rust
// ============================

// The basic structure of conditional statements in Rust:

// if condition {
//     // Code block executed if the condition is true
// } else if another_condition {
//     // Code block executed if the first condition is false and this condition is true
// } else {
//     // Code block executed if all previous conditions are false
// }

//function to return the bigger number!
//If both numbers are equal, any of them can be returned.
fn bigger_number (a: i32, b: i32) ->i32 {
    if a>b {
        a //return a
    }
    else if a<b {
        b //return b
    }
    else {
        a //return a as the condition says any number can be returned
    }

}

//condition checking with string:
fn foo_buzz (fruit: &str) -> &str{
    if fruit == "Mango" {
        "Foo"
    }
    else {
        "buzz"
    }
}

fn main (){
    let a: i32 = 44;
    let b: i32 = 33;
    let fruit: &str = "Apple";

    //calling the functions
    let bigger: i32 =  bigger_number(a,b);
    let foobuz: &str = foo_buzz (fruit);

    //printing the statements
    println! ("The bigger number is: {}", bigger);
    println! ( " {} ", &foobuz );

}