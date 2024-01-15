use std::io;
/*
DE:
    Schreibe ein Programm welches, den Preis der bestellten Pizza ermittelt
    Kosten:
    Pizza S = 3,50 €
    Pizza m = 5,50 €
    Pizza L = 7,00 €
    Pepperoni auf Pizza S = 2 €
    Pepperoni auf Pizza M, L = 3 €
    Extra Käse auf allen Pizzen = 1 €

EN:
    Write a program that determines the price of the ordered pizza
    Costs:
    Pizza S = 3,50 €
    Pizza m = 5,50 €
    Pizza L = 7,00 €
    Pepperoni on Pizza S = 2 €
    Pepperoni on Pizza M, L = 3 €
    Extra cheese on all pizzas = 1 €
*/

const PIZZA_PRICE_S: f32 = 3.5;
const PIZZA_PRICE_M: f32 = 5.5;
const PIZZA_PRICE_L: f32 = 7.0;
const PEPPERONI_PRICE_S: f32 = 2.0;
const PEPPERONI_PRICE_M: f32 = 3.0;
const PEPPERONI_PRICE_L: f32 = 3.0;
const CHEESE_PRICE: f32 = 1.0;
const YES: [&str; 4] = ["y", "yes", "ja", "j"];
fn main() {
    let size = read_input(&"Welche Größe soll die Pizza haben? (S, M, L)").to_uppercase();
    let pepperoni: bool = YES.contains(&read_input(&"Soll die Pizza Pepperoni haben? (y/n)").to_lowercase().as_str());
    let cheese: bool = read_input(&"Soll die Pizza extra Käse haben? (y/n)".to_string()).to_lowercase() == "y";

    let price = calculate_price(size, pepperoni, cheese);
    println!("Der Preis der Pizza beträgt: {} €", price);

}

fn calculate_price(size: String, pepperoni: bool, cheese: bool) -> f32 {
    let mut price: f32 = 0.0;

    if size == "S" {
        println!("Pizza S");
        price = PIZZA_PRICE_S;
        if pepperoni {
            price += PEPPERONI_PRICE_S;
        }
    } else if size == "M" {
        println!("Pizza M");
        price = PIZZA_PRICE_M;
        if pepperoni {
            price += PEPPERONI_PRICE_M;
        }
    } else if size == "L" {
        println!("Pizza L");
        price = PIZZA_PRICE_L;
        if pepperoni {
            price += PEPPERONI_PRICE_L;
        }
    } else {
        println!("Ungültige Größe");
    }

    if cheese {
        println!("Füge extra Käse hinzu");
        price += CHEESE_PRICE;
    }
    price
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
