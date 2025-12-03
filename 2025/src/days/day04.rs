use std::{fs, io::{BufReader, Read}};



/// # Panics
pub fn solve() {
    let file = fs::File::open("inputs/day04.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data).unwrap();
    println!("File: {data}");
}
