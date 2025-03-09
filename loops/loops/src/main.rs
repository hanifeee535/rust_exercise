/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/

/*
    rust loops
*/

// 1. FOR LOOP EXAMPLE
// A for loop is used to iterate over a range or collection.
// Syntax:
// for variable in collection/range {
//     // code block
// 

fn for_loop_example (){
    let numbers =[1,2,3,4,5];

    //itterating through the elements of the numbers array
    for num in numbers{
        println! (" {} ", num+1);
    }
}

// 2. WHILE LOOP EXAMPLE
// A while loop executes as long as a condition is true.
// Syntax:
// while condition {
//     // code block
// }

fn factorial (numb: i32) -> i32 {
    let mut factorial: i32 = 1 ;
    let mut i = 2;
    if numb == 0 || numb == 1 {
        return 1;
    }
    else {
        while i<=numb{
            factorial *= i;
            i +=1;
            
        }
        factorial

    }

    
    
}

fn main (){
    let factorial_result = factorial(5);
    println! (" factorial result =  {} ", factorial_result);
    for_loop_example();

}