use std::io;

fn main() {
    println! ("Restaurant Menu:");
    println!("P: Pounded Yam / Edinkaiko Soup - ₦3,200");
    println!("F: Fried Rice & Chicken - ₦3,000");
    println!("A: Amala & Ewedu Soup - ₦2,500");
    println!("E: Eba & Egusi Soup - ₦2,000");
    println!("W: White Rice & Stew - ₦2,500");

    println!("Enter food type (P/F/A/E/W):");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Not a valid string");
    let food_type = food.trim().to_uppercase();

    println!("Enter quantity:");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Not a valid string");
    let quantity: i32 = qty.trim().parse().expect("Not a valid number");

    let price: i32;
    if food_type == "P" {
        price = 3200;
    }
    else if food_type == "F" {
        price = 3000;
     } else if food_type == "A" {
        price = 2500;
     }else if food_type == "E" {
        price = 2000;
     } else if food_type == "W" {
        price = 2500;
     } else {
        println!("Invalid food type");
        return;
    }

    let mut total = price * quantity;
    if total > 10000 {
    let discount = (total as f32) * 0.05;
    total = total - discount as i32;
    println!("5% discount applied!");
    }

    println!("Total charge: N{}", total);
}