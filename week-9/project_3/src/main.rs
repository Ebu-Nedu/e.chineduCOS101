/*ALGO

1. Start.
2. Declare and initialize three vectors: commissioners, ministries, and zones.
3. Declare an empty vector called merged.
4. Initialize i = 0.
5. While i is less than the number of commissioners:\
   a. Combine the commissioner, ministry, and zone at index i into one record.\
   b. Add the record to merged.\
   c. Increment i by 1.
6. Create a file named EFCC_Merged.csv.
7. Write the column headings Commissioner, Ministry, and Zone to the file.
8. Write each record in merged to the file, one record per line.
9. Display a success message.
10. Stop.*/


use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = vec![
        "Aigbogun Akanbi Daudu",
        "Murtala Afeez Bendu",
        "Olorochia Callistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Chizuwa Faith Ebiye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut merged = Vec::new();

    for i in 0..commissioners.len() {
        let record = format!(
            "{},{},{}",
            commissioners[i],
            ministries[i],
            zones[i]
        );

        merged.push(record);
    }

    let mut file = File::create("EFCC_Merged.csv")
        .expect("Failed to create CSV file");

    writeln!(file, "Commissioner,Ministry,Zone")
        .expect("Failed to write header");

    for record in &merged {
        writeln!(file, "{}", record)
            .expect("Failed to write record");
    }

    println!("Records successfully merged into EFCC_Merged.csv");
}
