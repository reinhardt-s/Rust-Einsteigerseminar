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
````


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

