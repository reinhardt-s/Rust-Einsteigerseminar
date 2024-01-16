# Declarative macros with macro_rules!




The most common form of macro in Rust is the declarative macro. These are sometimes referred to as "macros by example", "macro_rules!" or simply "macros".  
At their core, declarative macros allow you to write something similar to a Rust `match` expression. `match` are control structures that take an expression, compare the resulting value of the expression to patterns, and then execute the code associated with the matching pattern.  
Macros also compare a value with patterns associated with specific code: In this situation, the value is the literal Rust source code passed to the macro; the patterns are compared to the structure of that source code; and the code associated with each pattern replaces the code passed to the macro if there is a match.  
**This all happens during compilation.**




## Creating your own macro

Using the example of logging, which takes a string and level parameters and outputs the entry in the console.




**A simple macro:**

```rust
#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        println!("{} - {}", file!(), $msg);
    };
    ($msg:expr, $level:expr) => {
        println!("{} - {} - {}", file!(), $level, $msg);
    };
}
```

**A more complex macro:**

> `#[macro_export]` is an attribute that makes the macro available for use in other files as soon as the containing crate is imported.

> `macro_rules!` is a macro that creates the definition of a macro.

> `$msg:expr` is a pattern that matches any expression and binds it as `$msg`.

https://doc.rust-lang.org/reference/macros-by-example.html


Calling the macro:

```rust
fn main() {
    log!("Hello World");
    log!("Hello World", "INFO");
}
````




```bash
src/main.rs - Hello World
src/main.rs - INFO - Hello World
```





### The vec! macro

An example of a macro that is included in the Rust standard library is the vec! macro. This macro takes any number of arguments and creates a vector from them.




```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```




`( $( $x:expr ),* ) => {` is a pattern that matches any number of expressions and binds them as `$x`.




`$(` and `)*` are so-called "repeaters" that repeat the content between them.  
`$x:expr` is a pattern that matches any expression and binds it as `$x`.




```rust
vec![1, 2, 3];
```




Generates the following code:




```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```




### Challenge:


**Challenge 1: (10 minutes)**  
Write a macro that:
 - takes a string to be printed
 - reads user input and returns it trimmed.



**Example call:**
```rust
fn main() {
    let msg = user_input!("Please enter your CC number: ");
    println!("{}", msg);
}
```




```bash
Please enter your CC number:
" XXX-XXX-XXX-XXX "
"XXX-XXX-XXX-XXX"
```
