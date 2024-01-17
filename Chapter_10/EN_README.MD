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

## Using `Result` and `Option`
The `Result` and `Option` types are used for functions that can fail or return nothing:

```rust
fn might_fail() -> Result<(), MyError> {
    Ok(())
}

fn might_return_nothing() -> Option<String> {
    Some("A string".to_string())
}
```

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

# Iterators, Map, Collect, and Enumerate

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