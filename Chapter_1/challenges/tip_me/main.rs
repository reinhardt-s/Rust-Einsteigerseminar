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

fn main() -> io::Result<()>{


    let stdin = io::stdin();

    let mut bill_input = String::new();
    let mut person_count_input = String::new();
    let mut tip_percentage_input = String::new();
    
    println!("Willkommen beim Tip Calculator!");

    println!("Wie hoch ist die Rechnung? ");
    stdin.read_line(&mut bill_input)?;

    println!("Wie viele Personen sind dabei? ");
    stdin.read_line(&mut person_count_input)?;

    println!("Wie viel Prozent Trinkgeld möchtest du geben? ");
    stdin.read_line(&mut tip_percentage_input)?;

    let bill: f32 = match bill_input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Rechnung konnte nicht ermittelt werden: {}", e);
            0.0
        },
    };
    let person_count: f32 = match person_count_input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Personeanzahl konnte nicht ermittelt werden: {}", e);
            1.0
        },
        };

    let tip_percentage: f32 = match tip_percentage_input.trim().parse(){
        Ok(num) => num,
        Err(e) => {
            panic!("Trinkgeld konnte nicht ermittelt werden: {}", e);
        },
    };




    let tip = bill * tip_percentage / 100.0;
    let total = bill + tip;
    let total_per_person = total / person_count;

    println!("Trinkgeld: {:.2}", tip);
    println!("Gesamt: {:.2}", total);
    println!("Gesamt pro Person: {:.2}", total_per_person);


    Ok(())
}
