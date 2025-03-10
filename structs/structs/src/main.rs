/*
    Author: Md Soyabbir Abu Hanif
    Email: hanifseceee535@gmail.com
*/

/*
    struct is the custom data type that groups related values under a single name
*/

//regular struct: 

struct Book {
    title: String,
    author: String,
    pages : u32,
    
}

//tuple struct:
struct ISBN (u64, u64, u64, u64);

//unit struct
struct Librarytag;

fn main() {

    let book = Book {
        title: String :: from ("The Rust Programming Language"),
        author: String ::from (" Someone name "),
        pages: 560,
    };
    let isbn = ISBN (978,1,59327, 8281);

    let tag = Librarytag;

    println!("\nTitle: {}, Pages: {} author: {} ", book.title, book.pages, book.author);
    println!("ISBN parts: {}-{}-{}-{}", isbn.0, isbn.1, isbn.2, isbn.3);
    
}
