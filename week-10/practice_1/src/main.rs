fn main() {

let v = vec![101, 250, 330, 400];
//vector v owns the object in heap

//only a single variable owns the heap memory at any give time
let v2 = v;
//here two variables owns the heap memory at any given time
//two pointers to the same content is not allowed in rust

//Rust i very smart in terms of memory access, so it detects a "race" as two variables to some heap

println!("{:?}", v);
}
