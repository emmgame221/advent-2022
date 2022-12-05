use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::VecDeque,
};

pub fn print_solution() {
    let input = File::open("inputs/input5.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    println!("Day 5 Part 1: {}", part_one(&lines));
    println!("Day 5 Part 2: {}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> String {
    let mut stacks: Vec<VecDeque<String>> = vec![VecDeque::new();9];
    for i in 0..8 {
        let line = &lines[i];
        let mut j = 1;
        let mut stack = 0;
        while j < line.len() {
            let c = &line[j..j+1];
            if c != " " {
                stacks[stack].push_front(c.to_owned());
            }
            stack += 1;
            j += 4;
        }
    }
    // Note: Input was edited by hand
    let mut i = 10;
    while i < lines.len() {
        /*for stack in stacks.iter() {
            print!("{:?}", stack.back());
        }
        println!();*/
        let mut parts = lines[i].split(' ');
        let num:usize = parts.next().unwrap().parse().unwrap();
        let from:usize = parts.next().unwrap().parse().unwrap();
        let to:usize = parts.next().unwrap().parse().unwrap();
        //println!("num:{} from:{} to:{} from_count:{} to_count:{}", num, from, to, stacks[from - 1].len(), stacks[to - 1].len());
        for _ in 0..num {
            if let Some(top) = stacks[from - 1].pop_back(){
                stacks[to - 1].push_back(top);
            }
        }
        i += 1;
    }
    let mut res = String::new();
    for stack in stacks {
        if let Some(s) = stack.back() {
            res.push_str(s);
        }
    }
    res
}

fn part_two(lines: &Vec<String>) -> String {
    let mut stacks: Vec<VecDeque<String>> = vec![VecDeque::new();9];
    for i in 0..8 {
        let line = &lines[i];
        let mut j = 1;
        let mut stack = 0;
        while j < line.len() {
            let c = &line[j..j+1];
            if c != " " {
                stacks[stack].push_front(c.to_owned());
            }
            stack += 1;
            j += 4;
        }
    }
    // Note: Input was edited by hand
    let mut i = 10;
    while i < lines.len() {
        /*for stack in stacks.iter() {
            print!("{:?}", stack.back());
        }
        println!();*/
        let mut parts = lines[i].split(' ');
        let num:usize = parts.next().unwrap().parse().unwrap();
        let from:usize = parts.next().unwrap().parse().unwrap();
        let to:usize = parts.next().unwrap().parse().unwrap();
        //println!("num:{} from:{} to:{} from_count:{} to_count:{}", num, from, to, stacks[from - 1].len(), stacks[to - 1].len());
        let mut tempstack: VecDeque<String> = VecDeque::new();
        for _ in 0..num {
            if let Some(top) = stacks[from - 1].pop_back(){
                tempstack.push_front(top);
            }
        }
        stacks[to - 1].append(&mut tempstack);
        i += 1;
    }
    let mut res = String::new();
    for stack in stacks {
        if let Some(s) = stack.back() {
            res.push_str(s);
        }
    }
    res
}