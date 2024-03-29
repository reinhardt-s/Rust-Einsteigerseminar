# Modules and Crates

Modules can be used to organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

## Creating a New Module

Creating an new module is done with the `mod` keyword, followed by the name of the module and a set of curly braces. Inside the curly braces, you can define items like functions, structs, enums, and other modules.

### Single File:

If your project is small, you can define modules in a single file (main.rs or lib.rs):

```rust
// In lib.rs or main.rs

mod my_module {
    pub fn my_function() {
        println!("Called my_function from my_module!");
    }
}

fn main() {
    my_module::my_function();
}
```

>**Note:** The keyword `pub` makes the function public, which means it can be used by outside code.

### Multiple Files:

If your project is large, you can define modules in multiple files:

```rust
// In my_module.rs

pub fn my_function() {
    println!("Called my_function from my_module!");
}
```

```rust
// In lib.rs or main.rs

mod my_module;

fn main() {
    my_module::my_function();
}
```

## Creating a librars crate with several modules

To have easy accessable, reusable code, you can create a library crate with several modules.

### Create a new library crate

```bash
cargo new my_library --lib
```

> **Note:** The `--lib` flag tells Cargo to create a library crate instead of a binary crate. Compare the Cargo.toml and lib.rs vs. main.rs files.


In the Cargo.toml file, you can specify some metadata about your crate:

https://doc.rust-lang.org/cargo/reference/cargo-targets.html#library
```toml
[lib]
name = "my_library"
crate-type = ["lib"]
```


### Declare and implement a module

```rust
// In src/lib.rs

pub mod module1;
pub mod module2;
```

```rust
// In my_library/src/module1.rs

pub fn function_in_module1() {
    // ...
}
```

## Using a module from another crate

### Using a module from a local crate

In your Cargo.toml file, add the following line to the `[dependencies]` section:

```toml
[dependencies]
my_library = { path = "../my_library" }
```

> **Note:** The `path` parameter tells Cargo to use the local crate instead of downloading it from crates.io.

In your code, you can now use the module:

```rust
use my_library::module1;

fn main() {
    module1::function_in_module1();
}
```

**Challenge:**  
Create a Rust library crate named arithmetic_ops that provides basic arithmetic operations (addition, subtraction, multiplication, division) for integers. Then, use this crate in a separate Rust binary project to demonstrate its functionality.


# Source Code in other files

The keyword `crate::` is used to start paths relative to the root of the current crate. It is used in particular in modules that are not the root module (e.g. main.rs or lib.rs) to refer to elements in the root module or in other modules.

See: https://doc.rust-lang.org/reference/items/modules.html#paths  
And also: https://doc.rust-lang.org/reference/paths.html#crate

## When to use crate::?

In non-root modules: In modules that are not directly defined in the root module, use crate:: to refer to elements defined in the root module or in sibling modules.
For clarity and to avoid naming conflicts: crate:: can also be used in the root module to create clarity or avoid naming conflicts with elements from external crates.

Imagine you have a crate structure as follows:

```bash
crate
├── main.rs (or lib.rs)
├── player.rs
└── game.rs
```

In game.rs to use a function from player.rs:

```rust
// In game.rs
use crate::player::some_function;

fn use_some_function() {
    some_function();
}
````
Here `crate::` enables access to some_function from the bar module.

