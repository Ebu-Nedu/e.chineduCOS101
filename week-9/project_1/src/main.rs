use std::io::Write;

fn main() {
    let drink = "Lager, Stout, Non-Alcoholic\n
33 Export, Legend, Maltina\n
Desperados, Turbo King, Amstel Malta\n
Goldberg, Williams, Malta Gold\n
Gulder, , Fayrouz\n
Heineken, , \n
Star, , ";

    let mut file = std::fs::File::create("NB_Drinks_Catalouge.csv").expect("Create Failed");
    file.write_all(drink.as_bytes()).expect("Write failed");
    
    println!("File Creation Successful!");
}
