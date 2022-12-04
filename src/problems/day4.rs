use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("inputs/input4.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let (p1, p2) = solve(&lines);
    println!("Day 4 Part 1: {}", p1);
    println!("Day 4 Part 2: {}", p2);
}

fn solve(lines: &Vec<String>) -> (i32, i32) {
    let mut tot1 = 0;
    let mut tot2 = 0;
    for line in lines {
        let (left, right) = line.split_once(',').unwrap();
        let (ll, lr) = left.split_once('-').unwrap();
        let leftstart: i32 = ll.parse().unwrap();
        let leftend: i32 = lr.parse().unwrap();
        let (rl, rr) = right.split_once('-').unwrap();
        let rightstart: i32 = rl.parse().unwrap();
        let rightend: i32 = rr.parse().unwrap();
        if (leftstart <= rightstart && leftend >= rightend)
            || (rightstart <= leftstart && rightend >= leftend)
        {
            tot1 += 1;
        }
        if (leftstart >= rightstart && leftstart <= rightend)
            || (rightstart >= leftstart && rightstart <= leftend)
        {
            tot2 += 1;
        }
    }
    (tot1, tot2)
}
