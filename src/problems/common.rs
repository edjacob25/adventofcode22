use std::fs::File;
use std::io::{read_to_string, BufReader};

pub fn read_file(file: &str) -> String {
    return read_to_string(BufReader::new(File::open(file).expect("day file missing")))
        .expect("Couldn't read");
}
