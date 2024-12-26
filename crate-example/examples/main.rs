#[derive(Debug, Clone)]
struct BookCollection(Vec<String>);

#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

impl Iterator for BookCollection {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop() {
            Some(book) => {
                println!("Accessing book: {book}");
                Some(book)
            }
            None => {
                println!("Out of books at the liarary!");
                None
            }
        }
    }
}

impl Library {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: BookCollection(Vec::new()),
        }
    }
    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }

    fn get_books(&self) -> BookCollection {
        self.books.clone()
    }
}

fn main() {
    let mut my_library = Library::new("Calgary");
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("蓝色潜水艇");
    my_library.add_book("黄色潜水艇");

    for book in my_library.get_books() {
        println!("{book}");
    }
}