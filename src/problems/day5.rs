use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("inputs/input5.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
}