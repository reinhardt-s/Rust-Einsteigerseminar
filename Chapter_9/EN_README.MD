# Testing

## 1. Test functions
Tests are Rust functions that are marked with the #[test] attribute. They are usually written in the same file as the code to be tested or in a parallel file structure under tests directories.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

## 2. Assert macros
Rust provides various macros for checking conditions:

assert!(expression): Checks if the expression is true.
assert_eq!(left, right): Checks for equality.
assert_ne!(left, right): Checks for inequality.

### 2.1 Partial equality
In Rust, PartialEq is a trait used to check partial equality between values. Unlike Eq, which implies a complete equality check (i.e., every comparison operation results in either true or false), PartialEq allows for an equality check that may not be defined or useful in all cases.

**Why PartialEq?**
A classic example of the use of `PartialEq` instead of Eq are floating point numbers (f32 and f64). Due to the way floating point numbers are represented, it is possible that two numbers that should be "equal" are actually considered unequal due to rounding errors. In addition, there are special values such as NaN ("Not a Number") where the usual logic of equality does not apply (NaN is by definition not equal to itself).

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
```

**Using PartialEq**
Once `PartialEq` is implemented for a type, you can use comparison operators such as == and != to compare instances of that type:

```rust
fn main() {
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 1, y: 2 };
    let point3 = Point { x: 3, y: 4 };

    assert_eq!(point1 == point2, true);
    assert_eq!(point1 == point3, false);
}
```

### 3. Test execution
Tests can be executed with the command cargo test. This command compiles the code in test mode and executes all functions marked with #[test].


### 4. Test configuration
With #[cfg(test)] you can specify that a module is only compiled in test mode so that the test code is not included in the final application or library.


### 5. Test results
After execution, Cargo displays a summary of the test results, including the number of tests performed, the number of successful tests and the number of failed tests.


### 6. Integration tests
In addition to unit tests, which test individual parts of the code, Rust also supports integration tests. These are usually in a dedicated tests directory and test the functionality of the code as a whole.


```rust
use my_crate;

#[test]
fn it_adds_two() {
    assert_eq!(my_crate::add_two(2), 4);
}
```


### Example
```rust
// calculator.rs


/// Adds two numbers and returns the result.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


/// Subtracts the second number from the first and returns the result.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}


/// Multiplies two numbers and returns the result.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}


/// Divides the first number by the second and returns the result.
/// Returns an error if the second number is zero.
pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero is not allowed.")
    } else {
        Ok(a / b)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 5), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err("Division by zero is not allowed."));
    }
}
```

#### Challenge:

```rust
// library.rs

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
```


**Beispiel mit Input**
```rust
macro_rules! readline {
    ($input:ident) => {
        std::io::stdin().read_line(&mut $input).unwrap();
    };
}


pub fn choose_answer() -> bool {
    let mut input = String::new();
    println!("What is the answer to life, the universe, and everything?");
    readline!(input); 

    process_answer(input.trim())
}

pub fn process_answer(answer: &str) -> bool {
    match answer {
        "42" => true,
        _ => false,
    }
}

pub fn choose_answer_with_input(input: &str) -> bool {
    process_answer(input.trim())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_answer_correct() {
        assert_eq!(process_answer("42"), true);
    }

    #[test]
    fn test_process_answer_incorrect() {
        assert_eq!(process_answer("not the answer"), false);
    }

    #[test]
    fn test_choose_answer_correct() {
        assert_eq!(choose_answer_with_input("42"), true);
    }
}
```
