/*ALGORITHM
Input experience status and age
↓
Check if employee is experienced
↓
Experienced → Check age
↓
Age ≥ 40 → Print ₦1,560,000
Age 30–39 → Print ₦1,480,000
Age 28–29 → Print "Unspecified!"
Age < 28 → Print ₦1,300,000

Not experienced → Print ₦100,000

Invalid experience status → Ask again*/

use std::io;

fn main() {
    let mut status = String::new();
    let mut age = String::new();

    let experienced: bool;

    loop {
        println!("\nExperienced or Not Experienced? ");
        io::stdin().read_line(&mut status).expect("Invalid Input");

        let status_input = status.trim().to_lowercase();

        if status_input == "experienced" {
            experienced = true;
            break;
        } else if status_input == "not experienced" {
            experienced = false;
            break;
        } else {
            println!("\nPlease type Experienced or Not Experienced");
            status.clear();
        }
    }

    if experienced {
        println!("\nEnter age: ");
        io::stdin().read_line(&mut age).expect("Invalid Input");
        let age: i32 = age.trim().parse().expect("Invalid Number");

        if age >= 40 {
            println!("\n₦1,560,000");
        } else if age >= 30 {
            println!("\n₦1,480,000");
        } else if age >= 28 {
            println!("\nUnspecified!");
        } else {
            println!("\n₦1,300,000");
        }
    } else {
        println!("\n₦100,000");
    }
}