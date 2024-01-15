# Allgemeine Konzepte

## Control Flow in Rust

Unter Control Flow versteht man die Kontrolle des Programmablaufs. In Rust gibt es dafür die folgenden Konstrukte:

- `if`-Ausdrücke
- `loop`-Ausdrücke
- `while`-Ausdrücke
- `for`-Ausdrücke

### `if`-Ausdrücke

`if`-Ausdrücke sind in Rust wie in den meisten anderen Programmiersprachen auch. Es gibt jedoch ein paar Unterschiede:

```rust
let number = 3;

if number < 5 {
    println!("number is smaller than 5");
} else {
    println!("number is greater than or equal to 5");
}
```

- Die Bedingung muss einen `bool` zurückgeben. Es ist nicht möglich, eine Zahl oder einen String zu übergeben.

- Die Bedingung muss nicht in Klammern stehen.

#### `if` in einem `let`-Statement

Es ist möglich, ein `if`-Statement in einem `let`-Statement zu verwenden:

```rust
let condition = true;
let number = if condition {
    5
} else {
    6
};
```

##### Inline-`if`-Statements

Es ist auch möglich, ein `if`-Statement inline zu verwenden:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

Hierbei muss jedoch beachtet werden, dass beide Zweige des `if`-Statements den gleichen Typ zurückgeben müssen. In diesem Fall ist es `i32`.

### Operatoren

| Operator | Bedeutung      |
----------|----------------
| `<`      | Kleiner        |
| `>`      | Größer         |
| `==`| Gleich         |
| `!=`| Ungleich       |
| `<=`     | Kleiner-Gleich |
| `>=`     | Größer-Gleich  |

### Boolean-Operatoren

Es gibt in Rust die folgenden Boolean-Operatoren:

- `&&` (AND)
- `||` (OR)
- `!` (NOT)

Mit diesen lassen sich mehrere Bedingungen verknüpfen:

```rust
let number = 3;

if number < 5 && number > 0 {
    println!("number is smaller than 5 and greater than 0");
} else {
    println!("number is greater than or equal to 5 or smaller than or equal to 0");
}
```

### `loop`-Ausdrücke

`loop`-Ausdrücke sind in Rust wie in den meisten anderen Programmiersprachen auch. Es gibt jedoch ein paar Unterschiede:

```rust
let mut counter = 0;

loop {
    counter += 1;

    if counter == 10 {
        break;
    }
}
```

- `loop`-Ausdrücke werden mit `loop` eingeleitet.

- `loop`-Ausdrücke können mit `break` beendet werden.

- Rückgabewerte von `loop`-Ausdrücken werden mit `break` zurückgegeben:
    
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

- Grundsätzlich beziehen sich `break`und `continue`auf die innerste Schleife.

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

- `loop`-Labels können verwendet werden, um `break`-Statements zu benennen:

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

> **'name:** Labels werden mit einem Apostroph eingeleitet. Der Name kann frei gewählt werden. Es ist jedoch üblich, den Namen in Kleinbuchstaben zu schreiben.

### `while`-Ausdrücke

Im Gegensatz zu `loop`-Ausdrücken werden `while`-Ausdrücke mit einer Bedingung eingeleitet:

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);

    number -= 1;
}

println!("LIFTOFF!!!");
```

#### Iteration mit `while`-Ausdrücken

Mit `while`-Ausdrücken können Iteratoren verwendet werden:

```rust
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    println!("the value is: {}", a[index]);

    index += 1;
}
```

### `for`-Ausdrücke

Mit `for`-Ausdrücken iterieren:

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {}", element);
}
```

#### Range und Reverse

Mit `for`-Ausdrücken können auch Ranges und Reverse-Ranges iteriert werden:

```rust

for number in (1..4).rev() {
    println!("{}!", number);
}

println!("LIFTOFF!!!");
```


### Zusammenfassung

- `if`-Statements sind Ausdrücke und können in `let`-Statements verwendet werden.

- `loop`-Statements sind Ausdrücke und können mit `break` einen Rückgabewert zurückgeben.

- `while`-Statements sind Ausdrücke und können mit Iteratoren verwendet werden.

- `for`-Statements sind Ausdrücke und können mit Iteratoren verwendet werden.

# Slices und Vectoren

## Listen

Listen sind eine Datenstruktur, die eine geordnete Sammlung von Werten enthält. Listen können nur Werte desselben Typs enthalten. Listen sind veränderbar.

### Erstellen einer Liste

```rust
let mut list = [1, 2, 3, 4, 5];
list[0] = 10;

println!("{:?}", list);

```

## Vektoren

Vektoren sind eine Datenstruktur, die eine geordnete Sammlung von Werten enthält. Vektoren können nur Werte desselben Typs enthalten. Vektoren sind veränderbar und können wachsen sowie schrumpfen.

### Erstellen eines Vektors

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

### Erstellen eines Vektor mit Macros

```rust
let mut vector = vec![1, 2, 3, 4, 5];

## Vector von Strings

```rust
let names = String::from("Alex, Sidney");
let mut vector = vec![String];

vector.push(names.split(", "));

println!("{:?}", vector);
```


# Challenges
Löse die folgenden Challenges:

- step_count (loop, read_line, std)
- leap_year (function, if, )
- pauls_pizza_palace(function, const, read_line, if)
- pay_roulette(read_line for, std::rand, vec)
- fizzbuzz (for, if)


