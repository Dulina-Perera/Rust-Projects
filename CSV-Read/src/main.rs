use std::error::Error;

use csv;

fn read_from_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = read_from_file("./customers.csv") {
        eprintln!("{}", err);
    }
}
