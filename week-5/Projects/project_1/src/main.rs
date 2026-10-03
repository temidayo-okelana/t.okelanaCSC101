use std::io::{self, Write};

fn main() {
    // display the Menu
    println!("Hello! This is our menu:");
    println!(" Code | Food Item                | Price");
    println!("  P   | Poundo Yam / Edinkaiko   | N3,200");
    println!("  F   | Fried Rice & Chicken     | N3,000");
    println!("  A   | Amala & Ewedu Soup       | N2,500");
    println!("  E   | Eba & Egusi Soup         | N2,000");
    println!("  W   | White Rice & Stew        | N2,500");

    // read the food code from customer
    print!("Enter food code (P, F, A, E, W): ");
    io::stdout().flush().unwrap();

    let mut food_code = String::new();
    io::stdin()
        .read_line(&mut food_code)
        .expect("Failed to read input");

    let food_code = food_code.trim().to_uppercase();

    // determine price per item based on selected food code
    let price: f64 = match food_code.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food code selected.");
            return;
        }
    };

    // read the quantity from customer
    print!("Enter quantity: ");
    io::stdout().flush().unwrap();

    let mut quantity_str = String::new();
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Failed to read input");

    let quantity: u32 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer for quantity.");
            return;
        }
    };

    // calculate total charge and discount
    let total_charge = price * (quantity as f64);
    let mut discount = 0.0;

    if total_charge > 10000.0 {
        discount = 0.05 * total_charge;
    }

    let final_amount = total_charge - discount;

    // output receipt summary
    println!("Receipt:");
    println!("Subtotal:       N{:.2}", total_charge);
    if discount > 0.0 {
        println!("Discount (5%):  -N{:.2}", discount);
    } else {
        println!("Discount:       N0.00");
    }
    println!("Total Payable:  N{:.2}", final_amount);
}