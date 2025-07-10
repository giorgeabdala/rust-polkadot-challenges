#![allow(dead_code)]
#![allow(unused_imports)]

pub struct Book {
    title: String,
    available: bool
}

impl Book {
    fn new(title: String)  -> Self{
        Book{title, available: true }
    }
}

pub struct Library {
     books: Vec<Book>
}

impl Library {
    fn new() -> Self {
        Library{books: Vec::new()}
    }
    
    // &mut self: exclusive access needed to modify internal state
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    
    // &self: shared access for read-only operations
    fn find_book(&self, title: &str) -> Option<&Book> {
       self.books.iter().find(|book| book.title == title) // Returns borrowed reference
    }
    
    fn borrow_book(&mut self, title: &str) -> bool {
       if let Some(book_to_borrow) = self.books.iter_mut().find(|book| book.title == title) {
         if book_to_borrow.available {
            book_to_borrow.available = false;
             return true
         }
       }
        false
    }
    
    
    fn return_book(&mut self, title: &str) -> bool {
        if let Some(book_to_return) = self.books.iter_mut().find(|book| book.title == title) {
            if !book_to_return.available {
                book_to_return.available = true;
                return true
            }
        }
        false
    }
    
}

mod tests {
    use crate::beginner::challenge_02::{Book, Library};

    #[test]
    fn find_book_test() {
        let mut library = Library::new();
        let title = "New Book".to_string();
        let book = Book::new(title.clone());
        let _ = Book::new("Book 2".to_string());
        library.add_book(book);
        let book_found_opt = library.find_book(&title);
        assert!(book_found_opt.is_some());
        assert_eq!(book_found_opt.unwrap().title, title);
    }
    
    #[test]
    fn borrow_book_test() {
        let mut library = Library::new();
        let title = "New Book".to_string();
        let book = Book::new(title.clone());
        library.add_book(book);
        assert!(library.find_book(&title).unwrap().available);
        let result = library.borrow_book(&title);
        assert!(result);
        assert!(!library.find_book(&title).unwrap().available);
    }
    
    #[test]
    fn return_book_test() {
        let mut library = Library::new();
        let title = "New Book".to_string();
        let book = Book::new(title.clone());
        library.add_book(book);
        let _ = library.borrow_book(&title);
        assert!(!library.find_book(&title).unwrap().available);
        
        let result = library.return_book(&title);
        assert!(result);
        assert!(library.find_book(&title).unwrap().available);
    }
    
    #[test]
    fn multiple_immutable_borrows_work() {
        let mut library = Library::new();
        let title = "The Rust Book".to_string();
        library.add_book(Book::new(title.clone()));

        let book_ref1 = library.find_book(&title);
        let book_ref2 = library.find_book(&title); // Another immutable borrow

        assert!(book_ref1.is_some());
        assert!(book_ref2.is_some());
        assert_eq!(book_ref1.unwrap().title, book_ref2.unwrap().title);
    }


}