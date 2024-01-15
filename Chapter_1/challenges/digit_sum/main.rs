/*
DE: Lasse die Nutzer*in eine zweistellige Zahl eingeben und gib dann die Quersumme aus.
EN: Let the user enter a two-digit number and then output the cross sum.
*/

use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();

    println!("Bitte gib eine Zahl ein:");
    stdin.read_line(&mut input)?;

    let first_digit_as_string = &input[0..1];
    let second_digit_as_string = &input[1..2];

    let first_digit = match first_digit_as_string.parse::<u32>() {
        Ok(number) => number,
        Err(_) => {
            println!("Die erste Stelle war keine Zahl!");
            return Ok(());
        }
    };
    let second_digit = match second_digit_as_string.parse::<u32>() {
        Ok(number) => number,
        Err(_) => {
            println!("Die zweite Stelle war keine Zahl!");
            return Ok(());
        }
    };

    let digit_sum = first_digit + second_digit;

    println!("Die Quersumme ist: {}", digit_sum);

    Ok(())
}
