# Asynchronous programming


Asynchronous functions in Rust are a powerful tool to efficiently handle multiple tasks at the same time, especially for operations that need to wait for external resources, such as network requests or file operations. Let me explain it to you step by step:


**What are asynchronous functions?**


Asynchronous functions allow your program to continue other tasks while waiting for a long-running operation to complete. Unlike synchronous code, which blocks and stops the entire program until a task is completed, asynchronous code can run in the background while the main part of your program continues to work.


**Why use Rust for asynchronous programming?


Rust provides a robust foundation for asynchronous programming through its memory safety model and concurrency capabilities. This means that you get high performance and security at the same time.


## Core concepts:


**Futures:** A future in Rust is a kind of placeholder for a value that might be available at some point in the future. Futures are the backbone of asynchronous programming in Rust.


`async` keyword: With `async` you can mark a function as asynchronous. An `async` function automatically returns a future.
`await` keyword: With `await` you can wait for the result of a future. It interrupts the execution of the current function until the future is ready and then continues it.
Example:


```rust
async fn tasks() {
    let result = other_asynchronous_function().await;
    // Process the result
}
```


In this example, `tasks` calls another asynchronous function and waits for the result with `await`.


**Executing asynchronous functions:**  
To execute asynchronous functions, you need a so-called executor that manages and executes the futures. Rust's standard library does not provide an executor, but there are external libraries such as `tokio` or async-std that do.


## Selection of different libraries for asynchronous programming


In Rust, asynchronous programming is a vibrant landscape with several libraries offering different approaches and functions. Three of the most prominent libraries in this area are Tokio, async-std and Mio.


   **Tokio**  
   Popular and widely used: Tokio is one of the best known and most widely used asynchronous runtime systems in the Rust community. It has a large user base and is used in many projects. Feature rich: Tokio offers an extensive collection of features, including TCP/UDP networking, file system operations, timing and more. It is particularly strong in networking and I/O related tasks. Powerful: Tokio is known for its performance and is well suited for high-load, scalable applications. Macro support: Tokio uses macros such as #[tokio::main] and #[tokio::test], which simplify its use in many cases. Learning curve: Due to its extensive functionality and peculiarities, Tokio can have a steeper learning curve for beginners.

   **async-std**  
   Ease of use: async-std aims to provide a simple and intuitive API that borrows heavily from Rust's standard library. This often makes it easier for beginners to get started. Performance: Although async-std is known for good performance, it is often considered slightly less powerful compared to Tokyo, especially in terms of certain network or I/O intensive tasks. Compatibility: async-std is often compatible with many futures and asynchronous ecosystems developed for Tokyo, making integration easier. Developer experience: The focus is on a clean and maintainable code base and a good developer experience.

   **Mio**  
   Low-level: Mio is a low-level event loop library that serves as the basis for other high-level libraries such as Tokio. It is designed to directly manage the core of an event loop with non-blocking I/O. Performance: Mio is known for its high performance as it directly accesses OS-level APIs such as epoll, kqueue, and IOCP. Fewer abstractions: Unlike tokyo or async-std, Mio offers fewer abstractions, which means more control, but also more complexity for the developer. Usage: Mio is often used as a foundation for other frameworks or for very specific use cases where direct control over the event loop is needed.


# Tokyo


Tokio is an asynchronous runtime environment for executing asynchronous code with Rust. Tokio provides an API for writing asynchronous code, a runtime environment that executes this code, and tools for debugging asynchronous code.


**Installation:**


```toml
[dependencies]
tokio = { version = "1.35.1", features = ["full"] }
futures = "0.3.30"
```


**Installation with Cargo**

```bash
cargo add tokio --features full
cargo add futures
```



**The Code:**
```rust
use rand::Rng;

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        let sleep_time = rand::thread_rng().gen_range(3..=5);
        println!("Task 1 sleeping for {} seconds", sleep_time);
        tokio::time::sleep(tokio::time::Duration::from_secs(sleep_time)).await;
        println!("Task 1 completed");
    });

    let task2 = tokio::spawn(async {
        let sleep_time = rand::thread_rng().gen_range(3..=5);
        println!("Task 2 sleeping for {} seconds", sleep_time);
        tokio::time::sleep(tokio::time::Duration::from_secs(sleep_time)).await;
        println!("Task 2 completed");
    });

    let task3 = tokio::spawn(async {
        let sleep_time = rand::thread_rng().gen_range(3..=5);
        println!("Task 3 sleeping for {} seconds", sleep_time);
        tokio::time::sleep(tokio::time::Duration::from_secs(sleep_time)).await;
        println!("Task 3 completed");
    });

    // Wait for all tasks to complete
    let _ = tokio::try_join!(task1, task2, task3);
    println!("All tasks completed");
}
```

**But what happens here?**

We use the `#[tokio::main]` macro to set up an asynchronous runtime.
Each task (task1, task2, and task3) is spawned using `tokio::spawn`. 
Inside each task, we simulate a delay with `tokio::time::sleep(tokio::time::Duration::from_secs(sleep_time)).await`. The `await` keyword yields control, allowing other tasks to run.

`rand::thread_rng().gen_range(3..=5)` generates a random number between 3 and 5 to use as the sleep duration.

The `tokio::try_join!` macro waits for all the tasks to complete. This is also an asynchronous operation, and it will yield control if the tasks are not yet completed.

**Challenge 1: (25 Minutes)**  
Erstelle ein Rust-Programm, das die Tokio-Bibliothek verwendet, um asynchron fünf verschiedene Zahlen auf Primzahleigenschaften zu prüfen.
Prüfen Sie alle fünf Zahlen unabhängig voneinander auf ihre Primzahleigenschaft und geben Sie die Ergebnisse aus, sobald sie berechnet sind.
