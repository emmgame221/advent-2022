use std::{
    fs::File,
    io::Read,
    collections::VecDeque,
};

pub fn print_solution() {
    let mut input = File::open("inputs/input6.txt").unwrap();
    let mut buf = String::with_capacity(5000);
    input.read_to_string(&mut buf).unwrap();
    let p1 = part_one(&buf);
    println!("Day 6 Part 1: {}", p1);
    let p2 = part_two(&buf);
    println!("Day 6 Part 2: {}", p2);
}

fn part_one(input: &String) -> i32 {
    let mut tot = 0;
    let mut window: VecDeque<char> = VecDeque::new();
    for c in input.chars() {
        tot += 1;
        if tot <= 4 {
            window.push_back(c);
        } else {
            window.pop_front();
            window.push_back(c);
        }
        if num_unique(&window) == 4 {
            return tot;
        }
    }
    tot
}

fn part_two(input: &String) -> i32 {
    let mut tot = 0;
    let mut window: VecDeque<char> = VecDeque::new();
    for c in input.chars() {
        tot += 1;
        if tot <= 14 {
            window.push_back(c);
        } else {
            window.pop_front();
            window.push_back(c);
        }
        if num_unique(&window) == 14 {
            return tot;
        }
    }
    tot
}

fn num_unique(lst: &VecDeque<char>) -> usize {
    let mut found: Vec<char> = Vec::new();
    //println!("{:?}", lst);
    for c in lst {
        if !found.contains(c) {
            found.push(*c);
        }
    }
    found.len()
}