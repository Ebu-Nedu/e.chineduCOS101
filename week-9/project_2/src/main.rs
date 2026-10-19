use std::io::Write;

fn main() {
    let sims = vec! [ 
    ("Student Name", "Matric. Number", "Department", "Level"),
    ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
    ("Adams Aliyu", "ECO10110101", "Economics", "100"),
    ("Shania Bolade", "CSC10328828", "Computer", "200"),
    ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
    ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];

    let mut file = std::fs::File::create("PAU_SIMS.csv").expect("Create failed");
    
    for row in sims.iter() 
{
    
    let line = row.0.to_string() + "," + row.1 + "," + row.2 + "," + row.3 + "\n";
    println!("{}",line);
    file.write_all(line.as_bytes()).expect("Write failed");
}
    

    println!("\nData written to file");

}
