/*/
DE:
Pay Roulette
 Schreibe ein Programm welches zufällig aus der Liste an Gästen einen auswählt,
 der die Rechnung zahlt. Gehe dazu wie folgt vor:
 Nimm eine Eingabe von Namen, mit Komma getrennt ("Hans, Magerete") via input() an
 Teile dann die den String in eine Liste auf
 Wähle dann, mithilfe von random eine Person aus, welche die Rechnung zahlt.
 Gebe dann mit einem f-String das Ergebnis aus
 Dein Code nach dieser Zeile

EN:
Pay Roulette
 Write a program that randomly selects one from the list of guests to pay the bill.
 Proceed as follows:
 Take an input of names, separated by commas ("Hans, Magerete") via input()
 Then divide the string into a list
 Then, using random, select a person to pay the bill.
 Then output the result with an f-string
 Your code after this line
*/

use std::io;
use rand::prelude::*;
fn main() {
    // read guests
    let names = input("Gib die Namen der Gäste ein, getrennt durch ', ': ");
    let mut vector: Vec<&str> = Vec::new();

    // split names
    let name_list = names.split(", ");
    for name in name_list {
        vector.push(name);
    }

    // randomly choose guest
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..vector.len());

    println!("{:#?}", vector);

    println!("{} zahlt die Rechnung!", vector[random_index]);
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => println!("Fehler beim Lesen der Eingabe: {}", e),
        
    }
    input.trim().to_string()
}