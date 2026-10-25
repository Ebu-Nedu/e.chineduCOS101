
use std::fs;
use std::io::Write;

fn main() {
    let file1 = fs::read_to_string("Commissioner.csv").unwrap();
    let mut commissioners = Vec::new();

    for line in file1.lines() {
        commissioners.push(line.trim_end_matches(',').to_string());
    }

    let file2 = fs::read_to_string("Ministry.csv").unwrap();
    let mut ministry = Vec::new();

    for line in file2.lines() {
        ministry.push(line.trim_end_matches(',').to_string());
    }

    let file3 = fs::read_to_string("Zone.csv").unwrap();
    let mut zone = Vec::new();

    for line in file3.lines() {
        zone.push(line.trim_end_matches(',').to_string());
    }

    let mut merged = Vec::new();

    for i in 0..commissioners.len() {
        let record = commissioners[i].to_string()
            + ","
            + &ministry[i]
            + ","
            + &zone[i]
            + "\n";

        merged.push(record);
    }

    let mut file = fs::File::create("EFCC_Merged.csv").unwrap();

    for record in &merged {
        file.write_all(record.as_bytes()).unwrap();
    }

    println!("Records successfully merged!");
}
