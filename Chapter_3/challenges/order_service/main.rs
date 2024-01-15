// Finde die Mutability-, Owner- und Borrowing-Fehler in diesem Programm


fn add_order(orders: mut Vec<i32>, id: i32) {
    &orders.push(id);
}

fn remove_order(orders: &mut Vec<i32>, id: i32) {
    let mut index_to_remove = None;
    for (index, order_id) in orders.iter().enumerate() {
        if order_id == id {
            index_to_remove = Some(index);
            break;
        }
    }

    if let Some(index) = index_to_remove {
        orders.remove(index);
    }
}

fn list_orders(orders: Vec<i32>) {
    for order in orders {
        println!("Bestellung ID: {}", order);
    }
}

fn main() {
    let mut orders = Vec::new();

    add_order(orders, 1);
    add_order(orders, 2);

    println!("Aktuelle Bestellungen:");
    list_orders(orders);

    remove_order(orders, 1);

    println!("Bestellungen nach dem Entfernen:");
    list_orders(&orders);
}
