/*
DE: Lasse die Nutzer*in Die Farbe des Lieblingstiers, sowie ein Schiffsteil eingeben und gib dann den Piratennamen aus.
EN: Let the user enter the color of their favorite animal and a part of a ship and then output the pirate name.
*/

use std::io;
use std::io::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut color = String::new();
    let mut part = String::new();

    println!("Ayeeee Mate, du bist also der neue Käpt'n 🏴‍☠️\n");

    println!("Welche Farbe hat dein Lieblingstier? 🦑");
    stdin.read_line(&mut color)?;
    // trim() entfernt Leerzeichen und Zeilenumbrüche
    color = color.trim().to_string();

    println!("Mit welchem Schiffsteil findest du dich am ehesten verbunden? 🚢");
    stdin.read_line(&mut part)?;
    part = part.trim().to_string();

    // Platzhalter {} werden durch Variablen ersetzt
    println!("Dein Piratenname ist Käpt'n {} {}", color, part);

    // Ok() gibt einen leeren io::Result zurück
    // ? gibt den Fehler zurück, falls einer aufgetreten ist
    Ok(())
}
