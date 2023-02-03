#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

// this function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// this function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // create an immutable Book named `immutabook`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;

    // immutably borrow an immutable object
    borrow_book(&immutabook);

    // immutably borrow a mutable object
    borrow_book(&mutabook);

    // borrow a mutable object as mutable
    new_edition(&mut mutabook);

    // error! cannot borrow an immutable object as mutable
    // new_edition(&mut immutabook);
}