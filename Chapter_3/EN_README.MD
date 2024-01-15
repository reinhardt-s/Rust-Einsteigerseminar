# Chapter 3


Ownership is one of the most important features of Rust. It allows Rust programs to run safely and without a garbage collector. In this chapter we will look at the basics of ownership.


## What is ownership?


Ownership is the most unique feature of Rust and has profound implications for the rest of the language. It allows Rust to provide memory safety guarantees without requiring a garbage collector, so it's important to understand how ownership works. In this chapter, we'll talk about ownership as well as some related features: borrowing, slices, and how Rust arranges data in memory.


> **Heap and Stack:**
> In Rust, memory and heap are used differently. The stack stores values in a "last-in, first-out" order and is fast because it uses fixed sizes. The heap is for variable-sized data and is slower as it is accessed via pointers. Ownership in Rust helps to manage the heap, allowing the language to provide memory safety without a garbage collector. Understanding ownership is crucial for efficient memory management in Rust.


## References


A reference is like a pointer in the sense that it is an address that we can follow to access the data stored at that address; that data belongs to another variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the lifetime of that reference.


## Rules of ownership


<img src="rusty.webp" alt="drawing" width="300" style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />


Rust has a few rules that govern the ownership system. These rules are:


- Every value in Rust has a variable that is its owner.
- There can only ever be one owner.
- If the owner leaves the scope, the value is dropped.


Simple example:


```rust
fn main() {
    let s = 4;
}
```


In the above example, `s` is the owner of the string literal `4`. If `s` leaves the scope, the string is deleted.  
`s` is in the stack because it has a fixed size.


## String type


The string type can be changed and therefore cannot be saved in the stack. It is stored in the heap.


```rust
fn main() {
    let s = String::from("hello");
}
```


`s`` is in the stack because it has a fixed size. The string literal `hello` is in the heap as it is mutable.


## Borrowing
<center>
<img src="stapler.gif" alt="drawing" height="200" style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />
</center>


Borrowing is a concept that makes it possible to reference data without taking ownership. It's like a pointer, but with a few extra rules that make it safer.


```rust
fn main() {
    let s = String::from("hello");


    let len = calculate_length(&s);


    println!("The length of '{}' is {}.", s, len);
}


fn calculate_length(s: &String) -> usize {
    s.len()
}
```


In this example, `s` is lent to the function `calculate_length`. The function can access `s`, but `s` remains in the possession of `main`.


## Slice type


A slice is a reference to a part of another data type. Slices are used to reference parts of strings.


```rust
fn main() {
    let s = String::from("hello world");


    let hello = &s[0..5];
    let world = &s[6..11];


    println!("{} {}", hello, world);
}
```


### Other slices


```rust
fn main() {
    let a = [1, 2, 3, 4, 5];


    let slice = &a[1..3];


    println!("{:?}", slice);
}
```

## Slices and vectors / lists


## Lists


Lists are a data structure that contains an ordered collection of values. Lists can only contain values of the same type. Lists can be modified.


### Creating a list


```rust
let mut list = [1, 2, 3, 4, 5];
list[0] = 10;


println!("{:?}", list);


```


## Vectors


Vectors are a data structure that contains an ordered collection of values. Vectors can only contain values of the same type. Vectors are mutable and can grow and shrink.


### Creating a vector


```rust
let mut vector : Vec<i32> = Vec::new();
vector.push(1);
vector.push(2);
vector.push(3);
vector.push(4);
vector.push(5);
vector[0] = 10;


println!("{:?}", vector);


vector.push(6);
vector.push(7);


println!("{:?}", vector);


vector.pop();
vector.remove(2);


println!("{:?}", vector);


```


### Create a vector with macros


```rust
let mut vector = vec![1, 2, 3, 4, 5];


## Vector of strings


```rust
let names = String::from("Alex, Sidney");
let mut vector = vec![String];


vector.push(names.split(", "));


println!("{:?}", vector);

```