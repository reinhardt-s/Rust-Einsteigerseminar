/*
DE:     Schreiben Sie ein Programm welche die Rechnung eines Tisches gleichmäßig auf alle Gäste aufteilt.
        Geben Sie das Ergebnis auf zwei Nachkommastellen gerundet aus.
        Berechnen Sie dann die Höhe des Trinkgeldes und geben Sie dann
        - den Gesamtbetrag pro Gast
        - den Rechnungsanteil ohne Trinkgeld pro Gast
        - das Trinkgeld pro Gast
        - das Gesamt-Trinkgeld
        in einer print()-Anweisung aus.

EN:     Write a program that evenly distributes the bill of a table to all guests.
        Round the result to two decimal places.
        Then calculate the amount of the tip and then give
        - the total amount per guest
        - the bill share without tip per guest
        - the tip per guest
        - the total tip
        in a print() statement.
*/

use std::io; // Import the io library

// Main function to start the program

fn main() -> io::Result<()> {
    let bill = 0;

    let tip = bill * tip_percentage / 100.0;
    let total = bill + tip;
    let total_per_person = total / person_count;

    println!("Tip: {:.2}", tip);
    println!("Total: {:.2}", total);
    println!("Total per guest: {:.2}", total_per_person);

    Ok(())
}
