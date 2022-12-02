use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/input1.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let elves = get_elves(&lines);
    let max = elves[elves.len() -  1];
    println!("Day 1 Part 1: {}", max);
    let top3 = elves[elves.len() -  3] + elves[elves.len() -  2] + elves[elves.len() -  1];
    println!("Day 1 Part 2: {}", top3);
}

fn get_elves(lines: &Vec<String>) -> Vec<i32> {
    let mut elves = Vec::new();
    let mut current = 0;
    for line in lines {
        if line == "" {
            elves.push(current);
            current = 0;
        } else {
            let val: i32 = line.parse().unwrap();
            current += val;
        }
    }
    elves.sort();
    elves
}