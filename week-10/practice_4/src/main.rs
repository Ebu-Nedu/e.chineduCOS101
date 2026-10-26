fn main() {
    // A list of numbers
    let v = vec![7, 19, 22];

    print_vector(v);

    println!("{:?}", v); // ERROR: v was moved into print_vector()
}

fn print_vector(x: Vec<i32>) {
    println!("{:?}", x);
}