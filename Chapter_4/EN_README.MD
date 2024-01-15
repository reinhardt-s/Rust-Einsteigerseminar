
# Rust Programming: Understanding Structs and Traits

## Structs in Rust

### What is a Struct?
- A **struct** (short for structure) in Rust is a custom data type that lets you name and package together multiple related values.
- It's similar to a record, struct in C, or class in other languages.

### Defining a Struct
- You define a struct with the `struct` keyword, followed by the struct's name and its fields in curly braces.
- Example:
  ```rust
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }
  ```

### Using Structs
- To use a struct, you create an instance of it and then access its fields using dot notation.
- Example:
  ```rust
  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };
  ```

## Traits in Rust

### What is a Trait?
- A **trait** in Rust is a way to define shared behavior in an abstract way.
- It can be compared to interfaces in other languages.

### Defining a Trait
- Traits are defined with the `trait` keyword, followed by the trait's name and a set of method signatures.
- Example:
  ```rust
  trait Summary {
      fn summarize(&self) -> String;
  }
  ```

### Implementing Traits
- To use a trait, you implement it for a specific struct.
- Example:
  ```rust
  impl Summary for NewsArticle {
      fn summarize(&self) -> String {
          format!("{}, by {} ({})", self.headline, self.author, self.location)
      }
  }
  ```
