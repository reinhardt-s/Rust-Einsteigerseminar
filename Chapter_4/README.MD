# Rust-Programmierung: Structs und Traits verstehen

## Structs in Rust

### Was ist eine Struktur?
- Eine **Struktur** (kurz für structure) in Rust ist ein benutzerdefinierter Datentyp, mit dem Sie mehrere zusammengehörige Werte benennen und zusammenfassen können.
- Es ist vergleichbar mit einem Datensatz, einer Struktur in C oder einer Klasse in anderen Sprachen.

### Definieren einer Struktur
- Eine Struktur wird mit dem Schlüsselwort `struct` definiert, gefolgt vom Namen der Struktur und ihren Feldern in geschweiften Klammern.
- Beispiel:
  ```rust
  struct Benutzer {
      username: String,
      email: String,
      sign_in_count: u64,
      aktiv: bool,
  }
  ```

### Structs verwenden
- Um eine Struktur zu verwenden, erstellen Sie eine Instanz davon und greifen dann über die Punktschreibweise auf ihre Felder zu.
- Beispiel:
  ```Rust
  let user1 = Benutzer {
      email: String::from("someone@example.com"),
      benutzername: String::from("someusername123"),
      aktiv: true,
      sign_in_count: 1,
  };
  ```

## Traits in Rust

### Was ist ein Trait?
- Ein **Trait** in Rust ist eine Möglichkeit, gemeinsames Verhalten auf abstrakte Art und Weise zu definieren.
- Man kann es mit Schnittstellen in anderen Sprachen vergleichen.

### Definieren eines Traits
- Traits werden mit dem Schlüsselwort "Trait" definiert, gefolgt von dem Namen des Traits und einer Reihe von Methodensignaturen.
- Beispiel:

  ```Rust
  trait Zusammenfassung {
      fn summarize(&self) -> String;
  }
  ```

### Implementieren von Traits
- Um einen Trait zu verwenden, implementieren Sie ihn für eine bestimmte Struktur.
- Beispiel:
  ```rust
  impl Zusammenfassung für NewsArtikel {
      fn summarize(&self) -> String {
          format!("{}, von {} ({})", self.headline, self.author, self.location)
      }
  }
  ```