// Define the dimensions of a rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// Logic to calculate the area of a rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        // Access fields using the self keyword
        self.width * self.height
    }
}

fn main() {
    // Instantiate the structure
    let small = Rectangle {
        width: 10,
        height: 20,
    };

    // Print the rectangle's dimensions and area
    println!(
        "Width is {}\nHeight is {}\nArea of Rectangle is {}",
        small.width,
        small.height,
        small.area()
    );
}

//Omo mehn holy speedrun