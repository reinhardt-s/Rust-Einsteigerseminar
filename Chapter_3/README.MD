# Kapitel 3

Ownership ist eine der wichtigsten Eigenschaften von Rust. Sie ermöglicht es, dass Rust Programme sicher und ohne Garbage Collector laufen können. In diesem Kapitel werden wir uns mit den Grundlagen von Ownership beschäftigen.

## Was ist Ownership?

Ownership ist das einzigartigste Merkmal von Rust und hat tiefgreifende Auswirkungen auf den Rest der Sprache. Es ermöglicht Rust, Garantien für die Speichersicherheit zu geben, ohne einen Garbage Collector zu benötigen, daher ist es wichtig zu verstehen, wie Ownership funktioniert. In diesem Kapitel werden wir über Ownership sowie einige damit verbundene Funktionen sprechen: das Ausleihen (Borrowing), Slices und wie Rust Daten im Speicher anordnet.

> **Heap und Stack:**
> In Rust werden Speicher und Heap unterschiedlich genutzt. Der Stack speichert Werte in einer "Last-in, First-out"-Reihenfolge und ist schnell, da er fixe Größen verwendet. Der Heap ist für variabel große Daten und ist langsamer, da er über Zeiger zugegriffen wird. Die Ownership in Rust hilft, den Heap zu verwalten, wodurch die Sprache Speichersicherheit ohne Garbage Collector bietet. Verständnis von Ownership ist für effiziente Speicherverwaltung in Rust entscheidend.

## Referenzen

Eine Referenz ist wie ein Zeiger in dem Sinne, dass es sich um eine Adresse handelt, der wir folgen können, um auf die an dieser Adresse gespeicherten Daten zuzugreifen; diese Daten gehören einer anderen Variablen. Im Gegensatz zu einem Zeiger ist eine Referenz garantiert auf einen gültigen Wert eines bestimmten Typs für die Lebensdauer dieser Referenz verweist.

## Regeln von Ownership

<img src="rusty.webp" alt="drawing" width="300" style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />

Rust hat ein paar Regeln, die das Ownership-System bestimmen. Diese Regeln sind:

- Jeder Wert in Rust hat eine Variable, die sein Owner ist.
- Es kann immer nur einen Owner geben.
- Wenn der Owner den Scope verlässt, wird der Wert gelöscht.

Einfaches Beispiel:

```rust
fn main() {
    let s = 4;
}
```

Im obigen Beispiel ist `s` der Owner des String-Literals `4`. Wenn `s` den Scope verlässt, wird der String gelöscht.  
`s` befindet sich im Stack, da es eine fixe Größe hat.

## String-Typ

Der String-Typ ist veränderbar und kann daher nicht im Stack gespeichert werden. Er wird im Heap gespeichert.

```rust
fn main() {
    let s = String::from("hello");
}
```

`s` befindet sich im Stack, da es eine fixe Größe hat. Der String-Literal `hello` befindet sich im Heap, da er veränderbar ist.

## Borrowing
<center>
<img src="stapler.gif" alt="drawing" height="200" style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />
</center>

Borrowing ist ein Konzept, das es ermöglicht, Daten zu referenzieren, ohne Ownership zu übernehmen. Es ist wie ein Zeiger, aber mit ein paar zusätzlichen Regeln, die es sicherer machen.

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

In diesem Beispiel wird `s` an die Funktion `calculate_length` ausgeliehen. Die Funktion kann auf `s` zugreifen, aber `s` bleibt im Besitz von `main`.

## Slice-Typ

Ein Slice ist ein Verweis auf einen Teil eines anderen Datentyps. Slices werden verwendet, um Teile von Strings zu referenzieren.

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);
}
```

### Andere Slices

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{:?}", slice);
}
```