# General concepts


## Control Flow in Rust


Control Flow is the control of the program flow. In Rust there are the following constructs for this:


- `if` expressions
- `loop` expressions
- `while` expressions
- `for` expressions


### `if` expressions


`if` expressions are the same in Rust as in most other programming languages. However, there are a few differences:


```rust
let number = 3;


if number < 5 {
    println!("number is smaller than 5");
} else {
    println!("number is greater than or equal to 5");
}
```


- The condition must return a `bool`. It is not possible to pass a number or a string.


- The condition does not have to be in brackets.


#### `if` in a `let` statement


It is possible to use an `if` statement in an `let` statement:


```rust
let condition = true;
let number = if condition {
    5
} else {
    6
};
```


##### Inline `if` statements


It is also possible to use an `if` statement inline:


```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```


However, it must be noted that both branches of the `if` statement must return the same type. In this case it is `i32`.


### Operators


| Operator | Meaning |
----------|----------------
| `<` | Smaller |
| `>` | Greater |
| `==`| Equal |
| `!=`| Unequal |
| `<=` | Smaller-Equal |
| `>=` | Greater-than-equal |


### Boolean operators


There are the following Boolean operators in Rust:


- `&&` (AND)
- `||` (OR)
- `!` (NOT)


These can be used to link several conditions:


```rust
let number = 3;


if number < 5 && number > 0 {
    println!("number is smaller than 5 and greater than 0");
} else {
    println!("number is greater than or equal to 5 or smaller than or equal to 0");
}
```


### `loop` expressions


`loop` expressions in Rust are like in most other programming languages. However, there are a few differences:


```rust
let mut counter = 0;


loop {
    counter += 1;


    if counter == 10 {
        break;
    }
}
```


- `loop` expressions are introduced with `loop`.


- `loop` expressions can be terminated with `break`.


- Return values of `loop` expressions are returned with `break`:
    
    ```rust
    let mut counter = 0;


    let result = loop {
        counter += 1;


        if counter == 10 {
            break counter * 2;
        }
    };


    println!("The result is {}", result);
    ```


- Basically, `break`and `continue` refer to the innermost loop.


``` rust
let mut outer_counter = 0;


loop {
    outer_counter += 1;


    let mut inner_counter = 0;


    loop {
        inner_counter += 1;


        if inner_counter == 10 {
            break;
        }
    }


    if outer_counter == 10 {
        break;
    }
}
```


- `loop` labels can be used to name `break` statements:


    ```rust
    'outer: loop {
        println!("Entered the outer loop");


        'inner: loop {
            println!("Entered the inner loop");


            break 'outer;
        }


        println!("This point will never be reached");
    }


    println!("Exited the outer loop");
    ```


> **'name:** Labels are introduced with an apostrophe. The name can be freely chosen. However, it is usual to write the name in lower case.


### `while` expressions


In contrast to `loop` expressions, `while` expressions are introduced with a condition:


```rust
let mut number = 3;


while number != 0 {
    println!("{}!", number);


    number -= 1;
}


println!("LIFTOFF!!!");
```


#### Iteration with `while` expressions


Iterators can be used with `while` expressions:


```rust
let a = [10, 20, 30, 40, 50];
let mut index = 0;


while index < 5 {
    println!("the value is: {}", a[index]);


    index += 1;
}
```


### `for` expressions


Iterate with `for` expressions:


```rust
let a = [10, 20, 30, 40, 50];


for element in a {
    println!("the value is: {}", element);
}
```


#### Range and reverse


Ranges and reverse ranges can also be iterated with `for` expressions:


```rust


for number in (1..4).rev() {
    println!("{}!", number);
}


println!("LIFTOFF!!!");
```




### Summary


- `if` statements are expressions and can be used in `let` statements.


- `loop` statements are expressions and can return a return value with `break`.


- `while` statements are expressions and can be used with iterators.


- `for` statements are expressions and can be used with iterators.


# Slices and vectors


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




# Challenges
Solve the following challenges:


- step_count (loop, read_line, std)
- leap_year (function, if, )
- pauls_pizza_palace(function, const, read_line, if)
- pay_roulette(read_line for, std::rand, vec)
- fizzbuzz (for, if)