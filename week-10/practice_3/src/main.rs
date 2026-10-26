fn main() {
    let v = vec![10, 13, 15]; // Vector v owns the object on the heap

    let v2 = v; // Ownership moves from v to v2

    let v2_return = display(v2); // Ownership moves into display()
                                // Then returns to v2_return

    println!("In main: {:?}", v2_return);
}

fn display(v: Vec<i32>) -> Vec<i32> {
    println!("Inside display: {:?}", v);

    v // Return ownership to the caller
}