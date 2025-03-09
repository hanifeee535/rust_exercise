/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/
/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com

    Rust Vectors:

    - A vector is a growable, contiguous collection of elements of the same type.
    - Vectors are similar to arrays, but they can dynamically resize when elements are added or removed.
    - They are stored on the heap, so their size doesn’t need to be known at compile time.

    Vector Structure:
    - Vec<T>: A vector holding elements of type T.
    - Elements are stored in contiguous memory, which makes access by index very fast.

    Creating a Vector:
    - Using the `vec!` macro: let v = vec![1, 2, 3];
    - Using Vec::new(): let mut v = Vec::new();
      Then push elements: v.push(4);

    Accessing Elements:
    - By index: v[0]
    - Using .get(): v.get(2)

    Iterating through Vectors:
    - With a for loop
    - Using iterators and methods like .map() or .filter()

    
*/


//a function that returns an array and a vector
fn array_and_vect()->([i32;5],Vec<i32>) {
    let a = [10,20,30,40,50]; //array
    //creating a vector with the same elements in the array:
    let v = vec![10,20,30,40,50];
    (a,v)

}

//a function that takes a slice of integers as input and
//returns a new vector which elements are multiplied by 2
fn vec_loop (input: &[i32]) -> Vec<i32>{
    let mut output = Vec::new();

    //itterate over the input slice
    for element in input {
        output.push(element*2);
    }
    output

}

//vector mapping
fn vec_map(input: &[i32]) -> Vec <i32> {
    //map each element to its value plus 1
    input.iter().map(|element|element +1).collect()
}


fn main() {
   let (array, vector) = array_and_vect();
   let numbers = [1,2,3,4];
   let doubled = vec_loop(&numbers);
   let map_result = vec_map(&vector);

   println!("Array: {:?} ", array);
   println!("vector: {:?} ", vector);
   println! ("Doubled numbers: {:?} ", doubled);
   println!("map vector elements: {:?} ", map_result);
}
