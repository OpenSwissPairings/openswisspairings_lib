use std::fs;

use openswisspairings_lib::Situation;

const FILE_NAME: &str = "test_file.trf";

fn main() {
    println!("In file {FILE_NAME}");

    let contents = fs::read_to_string(FILE_NAME);

    match contents {
        Ok(c) => println!("{:#?}", Situation::try_from(c)),
        Err(_) => eprintln!("Failed to open file"),
    }
}
