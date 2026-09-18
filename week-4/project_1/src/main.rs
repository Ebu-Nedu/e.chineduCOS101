/*Input A, B, C
↓
Check if a = 0
↓
Calculate D
↓
Check D
↓
D > 0 → calculate two roots
D = 0 → calculate one root
D < 0 → no real roots*/


use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nEnter value of a: ");
    io::stdin().read_line(&mut input1).expect("Invalid Input");
    let a:f32 = input1.trim().parse().expect("Invalid number");

    println!("\nEnter value of b: ");
    io::stdin().read_line(&mut input2).expect("Invalid Input");
    let b:f32 = input2.trim().parse().expect("Invalid number");

    println!("\nEnter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Invalid Input");
    let c:f32 = input3.trim().parse().expect("Invalid Number");

    if a == 0.0{
        println!("\nNot a quadratic equation!");
    }else {
        let d = b*b - 4.0 * a * c;

    if d > 0.0{
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);
        println!("\nThe roots are: {} and {}", x1, x2 ); 

    } else if d == 0.0{
        let x = -b / (2.0 * a);
        println!("\nThe root is: {}", x );

    } else {
        println!("\nNo real roots!");
    }

    }

    

}
