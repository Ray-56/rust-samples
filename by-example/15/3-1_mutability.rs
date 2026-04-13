#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in a read-only memory area
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function accepts a reference to the Book type
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function accepts a reference to a mutable Book type and changes the year `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // Create an immutable Book instance named `immutabook`
    let immutabook = Book {
        // String literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `immutabook` named `mutabook`
    let mut mutabook = immutabook;

    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    // Borrow a mutable object immutably
    borrow_book(&mutabook);

    // Mutably borrow a mutable object
    new_edition(&mut mutabook);

    // Report an error! Mutably cannot borrow an immutable object
    // new_edition(&mut immutabook);
}