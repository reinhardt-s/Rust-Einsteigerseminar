# Enums

Enums in Rust are types which can have multiple different variants.

## Defining an Enum
You define an enum using the enum keyword:

```rust
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
}
```

## Using an Enum

Enums are useful for pattern matching:

```rust
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        // Other patterns
    }
}
```


# Exceptions

In Rust, error handling is primarily done using the `Result` and `Option` enums, rather than exceptions as in many other languages.

## Creating Custom Error Types
To define custom error types, you typically create an enum:

```rust
enum MyError {
    IoError(std::io::Error),
    ParseError,
    // Other error types
}
```

You can also implement the `std::error::Error` trait for your custom error type.

## Using `Result`
The `Result` type are used for functions that can fail:

```rust
fn might_fail() -> Result<(), MyError> {
    Ok(())
}
```

With this, you now can use `match` to have a proper error handling.

## Full example
```rust
#[derive(Debug)]
enum CastError {
    InvalidFormat(String),
}

impl std::fmt::Display for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CastError::InvalidFormat(ref description) => write!(f, "Invalid format: {}", description),
        }
    }
}

impl std::error::Error for CastError {}
```

```rust
fn cast(s: &str) -> Result<i32, CastError> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(CastError::InvalidFormat(s.to_string())),
    }
}


fn main() {
    let results = vec![cast("123"), cast("abc")];

    for result in results {
        match result {
            Ok(number) => println!("Parsed number: {}", number),
            Err(e) => println!("Error: {}", e),
        }
    }
}
```

**In Test**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_invalid_format() {
        let result = cast("not a number");
        match result {
            Err(CastError::InvalidFormat(_)) => (), // Test passes
            _ => panic!("Expected CastError::InvalidFormat"),
        }
    }
}
```


# Closures
Closures  are anonymous functions you can save in a variable or pass as arguments to other functions.

## Creating a Closure
Closures are created using `||` syntax:

```rust
let add_one = |x: i32| x + 1;
```

## Using a Closure
You can use closures as arguments or in iterators:

```rust
let numbers = vec![1, 2, 3];
let incremented_numbers: Vec<_> = numbers.iter().map(|&x| x + 1).collect();
```

# Iterators, Map, Collect, Next, Filter, and Enumerate

Rust provides powerful iterator traits that allow for complex data manipulation in a concise and readable way. This document covers some of the most commonly used iterator methods: .iter(), .map(), .collect(), and .enumerate().

## .iter()

The `.iter()` method creates an iterator over the items in a collection like an array, vector, or slice.

```rust
let vec = vec![1, 2, 3, 4, 5];
let iter = vec.iter(); // Creates an iterator over `vec`
```

## .map() and collect()

`.map()` is used to transform each item of an iterator into another value. This method takes a closure that defines the transformation.


`.collect()` is used to convert an iterator into a collection such as a vector, hashmap, etc. It's often used at the end of an iterator chain to collect the results.


```rust
let vec = vec![1, 2, 3, 4, 5];
let squared: Vec<i32> = vec.iter().map(|&x| x * x).collect();
```

Here, .collect() gathers the squared numbers into a new vector.

## .enumerate()

`.enumerate()` adds a loop counter to an iterator. The method returns each item of the iterator along with its index.

```rust
let vec = vec!['a', 'b', 'c'];
for (index, value) in vec.iter().enumerate() {
    println!("{}: {}", index, value);
}
```


```bash
0: a
1: b
2: c
```

## .next()

`.next()` is a fundamental method in Rust's iterator trait. It is used to retrieve the next item from an iterator. 

### Basic Usage

When you call `.next()` on an iterator, it does two things:

It returns an `Option` that is `Some(value)` if there is a value to yield, or `None` if the iterator has been exhausted (i.e., there are no more items).
It advances the iterator to the next item:

```rust
let mut numbers = vec![1, 2, 3].into_iter();

assert_eq!(numbers.next(), Some(1));
assert_eq!(numbers.next(), Some(2));
assert_eq!(numbers.next(), Some(3));
assert_eq!(numbers.next(), None); // No more items
```

In this example, each call to .next() returns the next number in the vector until there are no more numbers, at which point it returns None.

How `.next()` works with loops

`.next()` is often used in conjunction with loops to process each item in an iterator:

```rust
let numbers = vec![1, 2, 3];
let mut iter = numbers.iter();

while let Some(number) = iter.next() {
    println!("{}", number);
}
```

This loop will print each number in the numbers vector. The while let construct is a convenient way to keep calling .next() until it returns None, at which point the loop exits.

## .filter()

`.filter()` is used to filter the items in an iterator. It takes a closure that returns a boolean value. If the closure returns true, the item is included in the iterator. If the closure returns false, the item is not included in the iterator.

The `.filter()` method takes a closure as an argument. This closure is the predicate function that decides whether an element should be included in the resulting iterator.

It returns a new iterator that yields only those elements for which the predicate returns true.
`.filter()` is **lazy**, meaning it doesn't actually filter the items until you iterate over them. This is why it's often used in combination with other iterator methods like `.collect()` to gather the results.



`.filter()` can be used with iterators over complex types as well. Consider a struct Product and filtering products based on some criteria:

```rust
struct Product {
    id: u32,
    name: String,
    price: f32,
}

let products = vec![
    Product { id: 1, name: "Product 1".to_string(), price: 10.0 },
    Product { id: 2, name: "Product 2".to_string(), price: 20.0 },
];

let expensive_products: Vec<Product> = products.into_iter().filter(|p| p.price > 15.0).collect();
```

Here, .filter() is used to create a new iterator that contains only the products with a price greater than 15.0.