/*# ALGORITHM: SHAPE CALCULATOR

1. Start the program.

2. Display the menu showing the available shapes and the option to exit.

3. Ask the user to enter the code of the shape they want to calculate.

4. If the input is not a number, display an error message and ask again.

5. If the user enters 0, display an exit message and stop the program.

6. If the user enters a code that is not on the menu, display an error message and show the menu again.

7. Ask the user how many of that shape they want to calculate.

8. Check that the number entered is a valid whole number greater than zero. If it is not, ask the user to enter it again.

9. For each shape requested:

   a. Ask the user to enter the dimensions needed for the shape.

   b. Check that each dimension is a valid number greater than zero. If not, ask for it again.

   c. Calculate the area, surface area or volume using the correct formula for the selected shape.

   d. Display the result.

10. After calculating all the requested shapes, display the menu again.

11. Repeat until the user chooses to exit.

12. Stop.*/

use std::f64::consts::PI;
use std::io;

fn read_positive_number(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();

        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Invalid Input");

        let result = input.trim().parse::<f64>();

        match result {
            Ok(value) if value > 0.0 && value.is_finite() => break value,
            Ok(_) => println!("Please enter a positive, finite number."),
            Err(_) => println!("Invalid input. Please input a number."),
        }
    }
}

fn read_positive_count(prompt: &str) -> i32 {
    loop {
        let mut input = String::new();

        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Invalid Input");

        let result = input.trim().parse::<i32>();

        match result {
            Ok(value) if value > 0 => break value,
            Ok(_) => println!("Please enter a number greater than zero."),
            Err(_) => println!("Invalid input. Please enter a whole number."),
        }
    }
}

fn trapezium() -> f64 {
    let h = read_positive_number("Please enter the value of height(h): ");
    let a = read_positive_number("Please enter the value of side 1(a): ");
    let b = read_positive_number("Please enter the value of side 2(b): ");

    let area = h / 2.0 * (a + b);
    area
}

fn rhombus() -> f64 {
    let d_1 = read_positive_number("Please enter the value of diagonal1: ");
    let d_2 = read_positive_number("Please enter the value of diagonal2: ");

    let area = (d_1 * d_2) / 2.0;
    area
}

fn parallelogram() -> f64 {
    let b = read_positive_number("Please enter the value of breadth(b): ");
    let h = read_positive_number("Please enter the value of height(h): ");

    let area = b * h;
    area
}

fn cube() -> f64 {
    let s = read_positive_number("Please enter the value of side(s): ");

    let area = 6.0 * s * s;
    area
}

fn cylinder() -> f64 {
    let r = read_positive_number("Please enter the value of radius(r): ");
    let h = read_positive_number("Please enter the value of height(h): ");

    let volume = PI * r * r * h;
    volume
}

fn main() {
    loop {
        println!("=== SHAPE CALCULATOR ===");
        println!("1. Trapezium");
        println!("2. Rhombus");
        println!("3. Parallelogram");
        println!("4. Cube");
        println!("5. Cylinder");
        println!("0. Exit");

        let s_code: i8 = loop {
            let mut input = String::new();

            println!("Select shape code: ");
            io::stdin().read_line(&mut input).expect("Invalid Input");

            let result = input.trim().parse::<i8>();

            match result {
                Ok(value) => break value,
                Err(_) => println!("Invalid input. Please enter a number."),
            }
        };

        match s_code {
            1 => {
                let multiplier = read_positive_count("How many trapeziums? ");

                for _i in 0..multiplier {
                    let t_area = trapezium();
                    println!("The area of trapezium = {}", t_area);
                }
            }

            2 => {
                let multiplier = read_positive_count("How many rhombi? ");

                for _i in 0..multiplier {
                    let r_area = rhombus();
                    println!("The area of rhombus = {}", r_area);
                }
            }

            3 => {
                let multiplier = read_positive_count("How many parallelograms? ");

                for _i in 0..multiplier {
                    let p_area = parallelogram();
                    println!("The area of parallelogram = {}", p_area);
                }
            }

            4 => {
                let multiplier = read_positive_count("How many cubes? ");

                for _i in 0..multiplier {
                    let c_area = cube();
                    println!("The surface area of cube = {}", c_area);
                }
            }

            5 => {
                let multiplier = read_positive_count("How many cylinders? ");

                for _i in 0..multiplier {
                    let c_volume = cylinder();
                    println!("The volume of cylinder = {}", c_volume);
                }
            }

            0 => {
                println!("Exiting Shape Calculator...");
                break;
            }

            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }
    }
}
