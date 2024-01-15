# Tag 1 - Einführung in Rust

* Kurze Einführung in Rust
* Installation von Rust
* Erstes Programm

## Was ist Rust und welche Vorteile bietet es?

Rust ist eine moderne Programmiersprache, die sich auf Sicherheit, Geschwindigkeit und parallele Ausführung konzentriert. Sie wurde von Mozilla Research entwickelt und erstmals im Jahr 2010 der Öffentlichkeit vorgestellt. Rust ist als eine Art Nachfolger von C und C++ gedacht, indem es eine ähnliche Leistung und Kontrolle über Systemressourcen bietet, aber mit einem stärkeren Fokus auf Sicherheit und Robustheit.

Hier sind einige der wichtigsten Vorteile von Rust:

### Speichersicherheit

Eines der Hauptziele von Rust ist es, Speichersicherheitsfehler zu eliminieren, die in vielen anderen Systemprogrammiersprachen üblich sind. Dies wird durch das Ownership-Modell erreicht, das sicherstellt, dass jeder Speicherbereich nur einen Besitzer hat, der für die Freigabe des Speichers verantwortlich ist. Dieses Modell verhindert Fehler wie dangling pointers oder race conditions, die in Sprachen wie C ohne sorgfältige Programmierung und Review entstehen können.

### Concurrency ohne Data Races

Rust ermöglicht es, mehrere Tasks parallel auszuführen, ohne sich Sorgen über Konflikte beim Zugriff auf gemeinsame Ressourcen machen zu müssen. Das Ownership- und Typsystem von Rust stellen zur Compile-Zeit sicher, dass unsichere Zustände, die zu sogenannten "data races" führen könnten, verhindert werden.

### Performance

Rust ist darauf ausgelegt, eine Leistung zu bieten, die mit C und C++ vergleichbar ist. Es gibt keine Garbage Collection und wenig Laufzeitüberprüfung, was bedeutet, dass Rust-Programme direkt auf der Hardware ausgeführt werden können und für systemnahe Programmierung geeignet sind.

### Modernes Tooling

Mit Cargo bietet Rust ein hervorragendes Paketmanagement- und Build-System, das Abhängigkeiten verwaltet, Tests durchführt, Dokumentation generiert und vieles mehr. Die Rust-Community stellt außerdem eine große Anzahl von "crates" (Paketen) zur Verfügung, die über crates.io zugänglich sind und die Entwicklung beschleunigen.

### Typsystem und Pattern Matching

Rusts Typsystem ist streng und hilft dabei, viele Arten von Fehlern zur Compile-Zeit zu fangen. Pattern Matching ist eine leistungsfähige Funktion von Rust, die es ermöglicht, auf elegante Weise komplexe Datenstrukturen zu durchlaufen und zu dekonstruieren.

### Interoperabilität

Rust kann gut mit C und anderen Sprachen zusammenarbeiten, was es ermöglicht, Rust-Code in bestehende Projekte zu integrieren oder Bibliotheken zwischen den Sprachen zu teilen.

### Aktive Community

Rust hat eine sehr aktive und engagierte Community. Die Sprache wurde mehrere Jahre in Folge zur "beliebtesten Sprache" unter Entwicklern gewählt, was unter anderem auf die freundliche Community und das starke Engagement für gute Entwicklungspraktiken zurückzuführen ist.

### Cross-Platform Development

Rust unterstützt Cross-Platform Development und kann auf einer Vielzahl von Betriebssystemen und Plattformen eingesetzt werden.

Diese Vorteile machen Rust zu einer attraktiven Wahl für viele Projekte, insbesondere solche, bei denen Sicherheit und Effizienz von entscheidender Bedeutung sind, wie Betriebssysteme, Spieleentwicklung, eingebettete Systeme und mehr.

## Installation von Rust

Wir halten uns an die offizielle Installationsanleitung von Rust, die unter https://doc.rust-lang.org/book/ch01-01-installation.html zu finden ist.

Als IDE verwenden wir VS-Code, da es eine gute Unterstützung für Rust bietet und eine der beliebtesten IDEs für Rust ist. VS-Code kann unter https://code.visualstudio.com/ heruntergeladen werden.

In VS-Code benötigen wir die folgenden Extensions:

* rust-analyzer
* CodeLLDB (für MacOS und Linux)
* Microsoft C/C++ Extension (für Windows)

## Erstes Programm

Um ein neues Programm mit Rust zu erstellen, verwenden wir das Cargo-Tool (https://doc.rust-lang.org/book/ch01-03-hello-cargo.html), das mit Rust installiert wird. Cargo ist das Build-System und Paket-Manager für Rust und wird für die meisten Rust-Projekte verwendet.

Um ein neues Projekt zu erstellen, führen Sie den folgenden Befehl aus:

```bash
cargo new hello_world
```

Dieser Befehl hat einen Ordner mit dem von uns angegebenen Namen erstellt.
Darin befinden sich folgende Dateien:

* Cargo.toml - Die Konfigurationsdatei für das Projekt
* .gitignore - Die Datei, die die Dateien und Ordner auflistet, die von Git ignoriert werden sollen
* src/main.rs - Die Hauptdatei des Projekts

Cargo.toml:

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

name - Der Name des Projekts  
version - Die Version des Projekts  
edition - Die Rust-Edition, die verwendet werden soll (2021 ist aktuell die neueste Edition)

In der Datei Cargo.toml werden auch die Abhängigkeiten des Projekts aufgelistet. Wir werden später sehen, wie wir Abhängigkeiten hinzufügen können.

#### Ausführen des Projekts - Als Debug-Version

Bevor wir das Programm ausführen können, muss es gebaut werden:

```bash
cargo build
```

Dieser Befehl erstellt eine ausführbare Datei im Ordner target/debug/hello_world. Diese Datei können wir ausführen:

```bash
./target/debug/hello_world
```

#### Erstellen eines Release-Builds

To create a new program with Rust, we use the Cargo tool (https://doc.rust-lang.org/book/ch01-03-hello-cargo.html), which is installed with Rust. Cargo is the build system and package manager for Rust and is used for most Rust projects.

To create a new project, execute the following command:

```bash
cargo new hello_world
```

This command creates a folder with the specified name.
It contains the following files:

* Cargo.toml - The project's configuration file
* .gitignore - The file listing files and folders to be ignored by Git
* src/main.rs - The project's main file

| usize | Vorzeichenloser Integer, Größe abhängig von der Architektur | let x: usize = 5; | 0 bis 4.294.967.295 (32-Bit) oder 0 bis 18.446.744.073.709.551.615 (64-Bit) | 4 Byte (32-Bit) oder 8 Byte (64-Bit) |
| f32 | 32-Bit Gleitkommazahl | let x: f32 = 5.0; | 1,2 * 10^-38 bis 3,4 * 10^38 | 4 Byte |
| bool | Boolean | let x: bool = true; | true oder false | 1 Byte |
| char | Unicode Zeichen | let x: char = 'a'; | 0 bis 1.112.064 | 4 Byte |
| str | String Slice | let x: &str = "Hello"; |  | 2 Byte pro Zeichen |

### Veränderbarkeit - mut

Variablen sind standardmäßig unveränderlich. Um eine Variable veränderlich zu machen, verwenden wir das Schlüsselwort mut.

```rust
let x = 5;
x = 6; // Fehler: x ist unveränderlich
println!("The value of x is: {x}");
```

```rust
let mut x = 5;
x = 6; // OK
println!("The value of x is: {x}");
```

#### It was the heap of the moment

<center>
    <img src="heat-of-the-moment-supernatural.gif" alt="drawing" width="300"
    style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />
</center>

Bevor wir uns mit Strings beschäftigen, müssen wir uns mit dem Heap und dem Stack vertraut machen. Der Stack ist ein Speicherbereich, der zur Laufzeit verwendet wird, um Variablen und Funktionen zu speichern. Der Heap ist ein Speicherbereich, der zur Laufzeit verwendet wird, um Daten zu speichern, deren Größe zur Compile-Zeit nicht bekannt ist oder die zur Laufzeit verändert werden können.

<center>
    <img src="Heap_Stack.png" alt="drawing" width="750"
    style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />
</center>

### String und &str

In Rust gibt es zwei Arten von Strings:  
`String` ist ein veränderlicher Datentyp in Rust, der auf dem Heap alloziert wird. Das bedeutet, dass er zur Laufzeit in der Größe verändert werden kann, indem man beispielsweise mehr Daten an das Ende des Strings anhängt.

`&str`, oft als "string slice" bezeichnet, ist eine unveränderliche Ansicht in einen String. Ein `&str` lebt in der Regel im Speicher des Programms, was den statisch allozierten Speicher für Literale einschließen kann, aber er kann auch auf einen Teil eines String oder eines anderen &str zeigen. Da `&str` ein unveränderlicher Verweis (borrow) ist, kann der Inhalt, auf den er zeigt, nicht verändert werden.

```rust
// Erstellen eines veränderlichen `String`-Objekts
let mut s1 = String::from("Hello");
// Anhängen eines Strings an `s1` mit `push_str`
s1.push_str(", world!");
println!("The value of s1 is: {}", s1); // Ausgabe: The value of s1 is: Hello, world!

// Erstellen eines unveränderlichen String-Literals `&str`
let s2 = "Hello";
println!("The value of s2 is: {}", s2); // Ausgabe: The value of s2 is: Hello

// Konvertieren eines `&str` in ein `String`-Objekt und Verändern
let mut s3 = s2.to_string(); // oder `let mut s3 = String::from(s2);`
// Anhängen eines Strings an `s3` mit `push_str`
s3.push_str(", Rust!");
println!("The value of s3 is: {}", s3); // Ausgabe: The value of s3 is: Hello, Rust!

// Verändern von `s3` durch Zuweisung eines neuen Wertes
s3 = "Goodbye, Rust!".to_string();
println!("The value of s3 is now: {}", s3); // Ausgabe: The value of s3 is now: Goodbye, Rust!
```

#### Das n-te Element eines Strings

```rust
let s = String::from("Hello, world!");
let second_element = &s[1..2];

let hello = &s[0..5]; // oder &s[..5]
let world = &s[7..12]; // oder &s[7..]
println!("The value of hello is: {}", hello); // Ausgabe: The value of hello is: Hello
println!("The value of world is: {}", world); // Ausgabe: The value of world is: world
println!("The value of second_element is: {}", second_element); // Ausgabe: The value of second_element is: e
```

**Und was passiert bei nicht-ASCII-Zeichen?**

```rust
let emoji_hell = String::from("👋🌎");
let second_element = &emoji_hell[1..2];
println!("The value of second_element is: {}", second_element); // Ausgabe: The value of second_element is: �
/*
byte index 1 is not a char boundary; it is inside '👋' (bytes 0..4) of `👋🌎`
*/
```

Bei nicht-ASCII-Zeichen kann es zu Problemen kommen, da diese mehr als ein Byte belegen können. Um dieses Problem zu lösen, können wir die Methode `chars()` verwenden, um über die Zeichen des Strings zu iterieren.

```rust
let emoji_hell = String::from("👋🌎");
let second_element = emoji_hell.chars().nth(1);
println!("The value of second_element is: {:?}", second_element); // Ausgabe: The value of second_element is: Some('🌎')
```

<img src="str_slice.png" alt="drawing" width="700"
style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />

> **Referenzen:** In Rust, wenn wir über Teile von komplexeren Datenstrukturen sprechen – wie etwa Strings –, ohne sie zu klonen oder zu verändern, verwenden wir in der Regel Referenzen. Das &-Symbol erstellt eine solche Referenz. Im Kontext von String-Slices bedeutet &s[1\..2] folgendes:  
> s ist ein String, dessen Inhalt im Heap gespeichert ist.
> [1\..2] definiert den Bereich des Strings, den wir erfassen möchten. Es gibt den Start- und den End-Index des gewünschten Slices an. Dieses spezielle Beispiel beginnt bei Index 1 und endet vor Index 2, was effektiv das zweite Byte des String auswählt.\
> & vor s[1\..2] sagt Rust, dass wir eine Referenz auf den Teil des Strings haben wollen, nicht den tatsächlichen Inhalt selbst kopieren. Das Erstellen eines Slices erzeugt immer eine "geliehene" Sicht auf einen Teil eines Strings und gibt uns keinen Besitz über die Daten.   Wir gehen später noch genauer auf Referenzen ein.

# ? Operator, Option und Result, unwrap und expect, match

## ? Operator

Der ? Operator ist eine Abkürzung für das Auspacken von Option und Result. Er kann nur in Funktionen verwendet werden, die Option oder Result zurückgeben.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("The value of s is: {}", s);
    Ok(())
}
```

## Option und Result

Option und Result sind Enumerations, die in der Standardbibliothek von Rust definiert sind. Sie werden verwendet, um Fehler und andere unerwartete Ergebnisse zu behandeln.

### Option

Option ist ein Enum, das entweder Some(x) oder None sein kann. Some(x) enthält einen Wert x, während None bedeutet, dass kein Wert vorhanden ist.

```rust
fn main() {
    let x = Some(5);
    let y: Option<i32> = None;
}
```

### Result

Result ist ein Enum, das entweder Ok(x) oder Err(e) sein kann. Ok(x) enthält einen Wert x, während Err(e) bedeutet, dass ein Fehler e aufgetreten ist.

```rust
use std::fs::File;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");
    let mut s = String::new();
    match f {
        // f == Ok(file)
        Ok(mut file) => {
            file.read_to_string(&mut s);
            println!("The value of s is: {}", s);
        }
        // f == Err(e)
        Err(e) => println!("Error: {}", e),
    }
}
```

## match

`match` ist ein Ausdruck, der verwendet wird, um einen Wert mit einer Reihe von Mustern zu vergleichen und dann den Code auszuführen, der mit dem ersten übereinstimmenden Muster übereinstimmt.

`.as_str()` wandelt einen String in einen String-Slice um. (`&str`)  
`.to_string()` wandelt einen String-Slice in einen String um. (`String`)

```rust
fn main() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }
}
```

```rust
fn main() {
    let x = 5;
    let y = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };
    println!("The value of y is: {}", y);
}
```

### unwrap und expect

\`unwrap\` ist eine Methode, die auf Option und Result aufgerufen werden kann. Sie gibt den Wert zurück, wenn es sich um Some(x) oder Ok(x) handelt, und löst einen Panic aus, wenn es sich um None oder Err(e) handelt.

```rust
fn main() {
	let x = Some(5);
    let y = x.unwrap();
    println!("The value of y is: {}", y);
}
```

```rust
fn main() {
    let x = Some(5);
    let y = x.expect("Failed to unwrap x");
    println!("The value of y is: {}", y);
}
```

### Don't panic!

<center>
    <img src="dont-panic-neil-degrasse-tyson.gif" alt="drawing" width="300"
    style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />
</center>

`panic!` ist ein Makro, welches einen Fehler auslöst und das Programm beendet. Es kann durch `unwrap` und `expect` ausgelöst werden.

```rust
fn main() {
    panic!("Something went wrong");
}
```

Oft geben Funktionen in Rust einen Wert vom Typ Result oder Option zurück. Diese Typen werden verwendet, um Fehler und andere unerwartete Ergebnisse zu behandeln.

## Option + Unwrap

Option ist ein Enum, das entweder Some(x) oder None sein kann. Some(x) enthält einen Wert x, während None bedeutet, dass kein Wert vorhanden ist.

```rust
fn main() {
    // second_element ist vom Typ Option<char>
    let mut second_element = emoji_hell.chars().nth(1);
    /*
    um den Wert von second_element zu erhalten, müssen wir ihn auspacken.
    */
    second_element = second_element.unwrap();
}
```

Unwraps sind böse, da sie einen Panic auslösen können, wenn der Wert None ist. Deshalb sollten wir sie nur verwenden, wenn wir sicher sind, dass der Wert nicht None ist.

`panic!` ist ein Makro, welches einen Fehler auslöst und das Programm beendet. Es kann mit `unwrap` und `expect` ausgelöst werden.


## Funktionen

Funktionen in Rust werden mit dem Schlüsselwort fn deklariert.

```rust
fn main() {
    println!("Hello, world!");
    another_one();
}

fn another_one() {
    println!("Biting dust...");
}
```

### Funktionsparameter

Funktionen können Parameter haben, die in der Signatur der Funktion angegeben werden. Die Parameter werden mit dem Namen und dem Typ deklariert. Der Typ kann weggelassen werden, wenn er vom Compiler ermittelt werden kann.

```rust
fn main() {
    let x = 5;
    let y = 6;
    let z = add(x, y);
    println!("The value of z is: {z}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

> **Hinweis:** In Rust gibt es keine Named-Parameter, d.h. die Reihenfolge der Parameter ist relevant.

### Statements und Expressions

In Rust gibt es zwei Arten von Codeblöcken: Statements und Expressions. Statements sind Anweisungen, die eine Aktion ausführen, aber keinen Wert zurückgeben. Expressions sind Codeblöcke, die einen Wert zurückgeben.

```rust
fn main() {
    let x = 5; // Statement
    let y = { // Expression
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}
```

### Rückgabewerte

Funktionen können einen Rückgabewert haben, der mit dem Schlüsselwort return zurückgegeben wird. Der Rückgabetyp wird mit dem Pfeil -> angegeben.

```rust
fn main() {
    let x = 5;
    let y = 6;
    let z = add(x, y);
    println!("The value of z is: {z}");
}

fn add(x: i32, y: i32) -> i32 {
    x++; // Fehler: x ist unveränderlich
    return x + y;
}
```

### Rückgabewerte - ohne return

Wenn eine Funktion kein return-Statement hat, wird der letzte Ausdruck in der Funktion als Rückgabewert verwendet. Das Schlüsselwort return kann also weggelassen werden.

```rust
fn main() {
    let x = 5;
    let y = 6;
    let z = add(x, y);
    println!("The value of z is: {z}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

Die Funktion `add` hat zwei Parameter vom Typ i32 und gibt einen Wert vom Typ i32 zurück. Die Funktion add wird mit den Parametern x und y aufgerufen und der Rückgabewert wird der Variablen z zugewiesen.

### Rückgabewerte - frühes Beenden

Eine Funktion kann auch vorzeitig beendet werden, indem das Schlüsselwort return ohne einen Wert verwendet wird. In diesem Fall wird die Funktion ohne Rückgabewert beendet.

```rust
fn main() {
    let x = 5;
    let y = 6;
    let z = add(x, y);
    println!("The value of z is: {z}");
}

fn add(x: i32, y: i32) -> i32 {
    if x > 5 {
        return x + y;
    }
    println!("x is not greater than 5");
}
```

## Shadowing und Scope

Variablen können in Rust überschattet werden. Das bedeutet, dass wir eine neue Variable mit demselben Namen wie eine bereits vorhandene Variable deklarieren können. Die neue Variable überschattet dann die alte Variable, d.h. wir können nur noch auf die neue Variable zugreifen.

Der Scope, also Geltungsbereich wird in Rust durch geschweifte Klammern definiert. Variablen, die innerhalb eines Scopes deklariert werden, sind nur innerhalb dieses Scopes gültig.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let class_id = 1433;

    {
        
        let x = x * 2; // Shadowing   
        let y = 10;     
        let class_id = "Rust#1433"; // Fehler: class_id ist bereits als i32 deklariert
        println!("The value of x in the inner scope is: {x}"); // x ist 12
        println!("The value of y in the inner scope is: {y}");
        println!("The value of class_id in the inner scope is: {class_id}");
    }

    println!("The value of x is: {x}"); // x ist wieder 6
    println!("The value of y is: {y}"); // Fehler: y ist nicht im Scope
    println!("The value of class_id is: {class_id}"); // class_id ist wieder 1433
}
```

## Kommentare

Kommentare in Rust werden mit // für einzeilige Kommentare und /* */ für mehrzeilige Kommentare geschrieben.

```rust
// Einzeiliger Kommentar

/*
Ein Kommentar
der über mehrere
Zeilen geht
*/

let rice_corn_count = 100; // Anzahl der Reiskörner


// Anzahl der Reissäcke
let sacks_of_rice = 20;
```

## Standard Bibliothek

Die Standardbibliothek von Rust bietet eine Reihe von Funktionen, die in jedem Rust-Programm verwendet werden können.
Für unseren Kurs, werden wir häufig mit der Standardbibliothek arbeiten.
Die wichtigsten Funktionen sind in im Kopf jedes Rust-Programms enthalten:

```rust
use std::collections::HashMap;
use std::io;
...
```

### :: - Operator

Der ::-Operator wird verwendet, um auf Funktionen und Typen zuzugreifen, die in einem Modul definiert sind. In diesem Fall greifen wir auf die Funktionen und Typen zu, die in der Standardbibliothek definiert sind.

### Modul - std

Die Standardbibliothek ist in Module unterteilt, die verschiedene Funktionen und Typen enthalten. Die wichtigsten Module sind:

* std::io - Ein- und Ausgabe
* std::collections - Sammlungen von Daten
* std::fs - Dateisystem
* std::net - Netzwerk
* std::thread - Threads
* std::time - Zeit

Ein Modul ist eine Sammlung von Funktionen und Typen, die in einer Datei oder einem Ordner definiert sind. In Rust werden Module verwendet, um Code zu organisieren und zu strukturieren. Module können auch verwendet werden, um Code zu verbergen, der nicht für die Verwendung außerhalb des Moduls gedacht ist.

Ein Modul wird mit dem Schlüsselwort mod deklariert. Der Code innerhalb des Moduls wird in geschweiften Klammern {} geschrieben.

```rust
mod my_module {
    // Code
}
```

### Standard Bibliothek - std::io

Die Standardbibliothek bietet eine Reihe von Funktionen für die Ein- und Ausgabe von Daten. Die (für uns) wichtigsten Funktionen und Makros sind:

* println! - Gibt eine Zeile auf der Konsole aus
* print! - Gibt eine Zeile auf der Konsole aus
* read_line - Liest eine Zeile von der Konsole ein

https://doc.rust-lang.org/std/io/struct.Stdin.html

```rust
use std::io;

fn read_food() -> io::Result<()>{
    let stdin = io::stdin();
    let mut food = String::new();


    println!("Was ist dein Lieblingsessen?");
    // read_line() liest eine Zeile von der Konsole ein und speichert sie in der Variable food
    
    stdin.read_line(&mut food)?;
    // trim() entfernt Leerzeichen am Anfang und Ende des Strings
    food = food.trim();

    // Platzhalter {} werden durch Variablen ersetzt
    println!("Dein Lieblingsessen ist {}", food);
    
    // Ok() gibt einen leeren io::Result zurück
    // ? gibt den Fehler zurück, falls einer aufgetreten ist
    Ok(())
}
```

#### <()>

```rust
fn main() -> io::Result<()>{
    // Code
    Ok(())
}
```

# Challenges
- pirate_name_generator(read_line)
- tip_me(read_line, basic arithmetic, )
- digit_sum(string slices, parse, Result)