// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
struct Book<'lifetime> {
    author: &'lifetime str,
    title: &'lifetime str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
