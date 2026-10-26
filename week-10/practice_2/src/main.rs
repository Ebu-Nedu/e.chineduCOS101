fn main() {
    let v = vec![2, 9, 10]; // Vector v owns the object on the heap

    let v2 = v; // Ownership moves from v to v2
                // v is no longer valid here

    display(v2); // Ownership moves from v2 to display()
                 // v2 is no longer valid here

    println!("In main: {:?}", v2); // ERROR: v2 was moved into display()
}

fn display(v: Vec<i32>) {
    println!("Inside display: {:?}", v);
}