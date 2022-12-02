use std::{io::{BufReader, BufRead}, fs::File};

pub fn print_solution() {
    let input = File::open("inputs/input3.txt").unwrap();
    let lines = BufReader::new(input).lines();
}