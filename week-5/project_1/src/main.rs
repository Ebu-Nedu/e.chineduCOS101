/*
ALGORITHM

Start
Display the restaurant menu.
Set total to 0.
Ask the customer to enter the food type.
Read the food type.
Check the food type:
If P, set price to 3200.
If F, set price to 3000.
If A, set price to 2500.
If E, set price to 2000.
If W, set price to 2500.
Otherwise, display Invalid food type.
Ask the customer to enter the quantity.
Read the quantity.
Calculate the order total:
order_total = price × quantity
Add the order total to the grand total:
total = total + order_total
Ask the customer if they want to order more.
If the answer is Y, go back to Step 4.
If the answer is N, check the grand total.
If total > 10000, calculate the discount:
discount = total × 5 / 100
Subtract the discount from the total:
final_total = total - discount
Display the total, discount, and final charge.
If total is not greater than 10000, display the total charge without a discount.
Display the thank-you message.
Stop.*/

use std::io;

fn main() {
    println!("================ RESTAURANT MENU ================");
    println!("P  Poundo Yam / Edinkaiko Soup    N3,200");
    println!("F  Fried Rice & Chicken           N3,000");
    println!("A  Amala & Ewedu Soup             N2,500");
    println!("E  Eba & Egusi Soup               N2,000");
    println!("W  White Rice & Stew              N2,500");
    println!("==================================================");

    println!("Welcome to Ebube's Chow Center!");

    let mut total: u32 = 0;

    loop {
        println!("Enter food type: ");

        let mut food_type = String::new();
        io::stdin()
            .read_line(&mut food_type)
            .expect("Invalid Input!");

        let food_type = food_type.trim().to_uppercase();

        let price: u32;

        if food_type == "P" {
            price = 3200;
        } else if food_type == "F" {
            price = 3000;
        } else if food_type == "A" {
            price = 2500;
        } else if food_type == "E" {
            price = 2000;
        } else if food_type == "W" {
            price = 2500;
        } else {
            println!("Invalid food type.\nPlease input correct food type.");
            continue;
        }

        println!("Enter quantity: ");

        let mut quantity = String::new();
        io::stdin()
            .read_line(&mut quantity)
            .expect("Invalid Input!");

        let quantity: u32 = quantity.trim().parse().expect("Invalid Input!");

        let order_total = price * quantity;
        total = total + order_total;

        println!("Order more? \nY/N");

        let mut order = String::new();
        io::stdin()
            .read_line(&mut order)
            .expect("Invalid Input!");

        let order = order.trim().to_uppercase();

        if order == "N" {
            break;
        }
    }

    if total > 10000 {
        let discount = total * 5 / 100;
        let final_total = total - discount;

        println!(
            "Total amount before discount: {}\nDiscount = {}\nTotal charge: {}",
            total, discount, final_total
        );
    } else {
        println!("Total charge: {}", total);
    }

    println!(
        "Thanks for your patronage! \nCome again! \nEbube's Chow! You can only say WOW!"
    );
}