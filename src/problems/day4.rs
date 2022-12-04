use std::{io::{BufReader, BufRead}, fs::File};

pub fn print_solution() {
    let input = File::open("inputs/input2.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
}