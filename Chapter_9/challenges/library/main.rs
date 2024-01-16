pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn remove_book(&mut self, title: &str) -> Result<(), String> {
        let index = self.books.iter().position(|b| b.title == title);

        match index {
            Some(i) => {
                self.books.remove(i);
                Ok(())
            },
            None => Err(format!("Book with the title '{}' not found.", title)),
        }
    }

    pub fn get_books_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    pub fn get_book_by_title(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.title == title)
    }
}