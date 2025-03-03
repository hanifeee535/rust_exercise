/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/

fn main (){
    /*
        Integer type: 
        Signed : i8, i16, i32, i64, i128, isize
        Unsigned: u8, u16, u32, u64, u128, usize
        The number indicates the bit size.

     */    

    let signed_integer: i32 = -42; //signed 32 bit integer
    let unsigned_integer : u32 = 42; //32 bit unsigned integer 

    /*
        Floating point: f32 and f64 for 32 bit and 64 bit floating point numbers

     */
    let float_32 : f32 = 3.14 ;  //32 bit floating point number
    let float_64 : f64 = 2.71828; //64 bit floating point number

    /*
        Boolean: bool with values true or false
     */
    let is_rust_hard : bool = false;

    /*
        character: char for a  single unicode scalar value (4 bytes)
        It can store any valid Unicode character, such as letters, symbols, or emojis.
         Unlike some languages, Rust's `char` is not limited to ASCII and uses 4 bytes for Unicode support.
     */
    let letter : char = 'A'; //Single unicode character

    //print all scaller type data: 
    println! ("Signed Integer: {}", signed_integer);
    println! ("Unsigned Integer: {}", unsigned_integer);
    println! ("32 bit float:  {}", float_32);
    println! ("64 bit float:  {}", float_64);
    println! ("Is Rust Hard?  {}", is_rust_hard);
    println! ("Character:  {}", letter);

}