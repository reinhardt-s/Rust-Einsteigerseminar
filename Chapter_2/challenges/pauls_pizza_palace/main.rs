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
fn main() {
    println!("Welcome to Paul's Pizza Palace!");
}

fn calculate_price(size: String, pepperoni: bool, cheese: bool) -> f32 {}

fn read_input(prompt: &str) -> String {}
