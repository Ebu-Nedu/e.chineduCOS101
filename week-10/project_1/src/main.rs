/*START
1. Create a struct called Lappy to store the brand, quantity and price of each laptop.
2. Create a cost() method to calculate the total cost of laptops purchased per brand.
3. Create an individual_cost() method to return the price of one laptop.
4. Create an opening() function to welcome the customer to Alaba Electronics.
5. Create a closing() function to thank the customer after the transaction.
6. Create four Lappy objects representing HP, IBM, Toshiba and Dell laptops.
7. Set the quantity purchased for each brand to 3 and assign the correct unit price.
8. Call opening() to display the welcome message.
9. Display the individual price of one laptop for each brand.
10. Use cost() to calculate and display the cost of purchasing 3 laptops from each brand.
11. Add the four purchase costs together and store the result in total_cost.
12. Display the total cost of the purchase.
13. Call closing() to display the farewell message.

STOP*/

struct Lappy {
    brand : String,
    quantity : u32,
    price : u32,
}

impl Lappy {
    fn cost(&self) -> u32 {
        self.quantity * self.price
    }

    fn individual_cost(&self) -> u32 {
        self.price
    }
}

fn opening() {
    println!("\nWelcome to Alaba Electronics!");
    println!("Thank you for choosing us.");
    println!("Here is your cost breakdown:");
}

fn closing() {
    println!("\nThank you for shopping with Alaba Electronics!");
    println!("We hope to serve you again.");
}

fn main() {
    let lappy_1 = Lappy {
        brand : "HP".to_string(),
        quantity : 3,
        price : 650000,
    };

    let lappy_2 = Lappy {
        brand : "IBM".to_string(),
        quantity : 3,
        price : 755000,
    };

    let lappy_3 = Lappy {
        brand : "Toshiba".to_string(),
        quantity : 3,
        price : 550000,
    };

    let lappy_4 = Lappy {
        brand : "Dell".to_string(),
        quantity : 3,
        price : 850000,
    };

    let total_cost = lappy_1.cost() + lappy_2.cost() + lappy_3.cost() + lappy_4.cost();

    opening();

    println!("\nCost of {} = {}",lappy_1.brand,lappy_1.individual_cost());
    println!("Cost of {} = {}",lappy_2.brand,lappy_2.individual_cost());
    println!("Cost of {} = {}",lappy_3.brand,lappy_3.individual_cost());
    println!("Cost of {} = {}",lappy_4.brand,lappy_4.individual_cost());

    println!("\nCost of 3 {} laptops = {}",lappy_1.brand,lappy_1.cost());
    println!("Cost of 3 {} laptops = {}",lappy_2.brand,lappy_2.cost());
    println!("Cost of 3 {} laptops = {}",lappy_3.brand,lappy_3.cost());
    println!("Cost of 3 {} laptops = {}",lappy_4.brand,lappy_4.cost());



    println!("\nCost of 3 laptops from each brand is  = {}",total_cost);

    closing();
    }
