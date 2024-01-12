use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use csv::Error;

fn main() -> Result<(), Error> {
    // Get file name from the command line arguments
    println!("Hello, world!");

    // static testing file name
    let path = Path::new("Discover-RecentActivity-20240104.csv");
    let display = path.display();

    // Open the file
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {})", display, why),
        Ok(file) => file,        
    };

    // read file into a string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contents\n{}", display, s)
    };

    let mut reader = csv::Reader::from_reader(s.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "A: {}, B: {}, C: {}, D: {}",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        )
    }

    Ok(())

    // Read in first line to get existing order

    // create standard homebank order to compare

    // map the existing rows into the homebank order

    // save the resulting map to a new file in the same directory

}
