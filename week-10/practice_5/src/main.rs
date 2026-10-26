fn main() {
    // A list of numbers
    let x = vec![27, 28];

    borrow_vector(&x); // Passing a reference

    println!("Printing the value from main() x = {:?}", x);
    println!("********************************");
}

fn borrow_vector(z: &Vec<i32>) {
    println!("Printing the value from borrow_vector() z = {:?}", z);
}