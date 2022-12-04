use std::{io::{BufReader, BufRead}, fs::File, collections::HashSet};

pub fn print_solution() {
    let input = File::open("inputs/input3.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let p1 = part_one(&lines);
    println!("Day 3 Part 1: {}", p1);
    let p2 = part_two(&lines);
    println!("Day 3 Part 2: {}", p2);
}

fn part_one(lines: &Vec<String>) -> i32 {
    let mut tot = 0;
    for line in lines {
        let midpoint = line.len() / 2;
        let (left, right) = line.split_at(midpoint);
        let mut leftset = HashSet::new();
        let mut rightset = HashSet::new();
        for c in left.chars() {
            leftset.insert(c);
        }
        for c in right.chars() {
            rightset.insert(c);
        }
        let res = leftset.intersection(&rightset);
        for c in res {
            tot += priority(*c);
        }
    }
    tot
}

fn part_two(lines: &Vec<String>) -> i32 {
    let mut linesiter = lines.into_iter();
    let mut tot = 0;
    loop {
        let elf1 = linesiter.next();
        if elf1 == None {
            break;
        }
        let elf1 = elf1.unwrap();
        let elf2 = linesiter.next().unwrap();
        let elf3 = linesiter.next().unwrap();
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        let mut set3 = HashSet::new();
        for c in elf1.chars() {
            set1.insert(c);
        }
        for c in elf2.chars() {
            set2.insert(c);
        }
        for c in elf3.chars() {
            set3.insert(c);
        }
        let res = set1.intersection(&set2);
        let mut set4 = HashSet::new();
        for c in res {
            set4.insert(*c);
        }
        let res = set4.intersection(&set3);
        for c in res {
            tot += priority(*c);
        }
    }
    tot
}

fn priority(c: char) -> i32 {
    if 'a' <= c && c <= 'z' {
        c as i32 - 'a' as i32 + 1
    } else if 'A' <= c && c <= 'Z' {
        c as i32 - 'A' as i32 + 27
    } else {
        panic!("Invalid character: {} (Day 3 priority())", c);
    }
}