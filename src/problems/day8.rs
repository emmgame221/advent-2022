use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("inputs/input8.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let trees = parse(&lines);
    let p1 = part_one(&trees);
    println!("Day 8 Part 1: {}", p1);
    let p2  = part_two(&trees);
    println!("Day 8 Part 2: {}", p2);
}

fn parse(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut trees: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for s in line.chars() {
            let t = s.to_string().parse().unwrap();
            row.push(t);
        }
        trees.push(row);
    }
    trees
}

fn part_one(trees: &Vec<Vec<i32>>) -> i32 {
    let mut tot = 0;
    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            if row == 0 || col == 0 || row == trees.len() - 1 || col == trees[row].len() - 1 {
                tot += 1;
            } else {
                let mut visible_west = true;
                let mut visible_east = true;
                let mut visible_north = true;
                let mut visible_south = true;
                let mut i = 0;
                while i < col {
                    if trees[row][i] >= trees[row][col] {
                        visible_west = false;
                        break;
                    }
                    i += 1;
                }
                i = trees[row].len() - 1;
                while i > col {
                    if trees[row][i] >= trees[row][col] {
                        visible_east = false;
                        break;
                    }
                    i -= 1;
                }
                i = 0;
                while i < row {
                    if trees[i][col] >= trees[row][col] {
                        visible_north = false;
                        break;
                    }
                    i += 1;
                }
                i = trees.len() - 1;
                while i > row {
                    if trees[i][col] >= trees[row][col] {
                        visible_south = false;
                        break;
                    }
                    i -= 1;
                }
                let visible = visible_east || visible_west || visible_north || visible_south;
                if visible {
                    tot += 1;
                }
            }
        }
    }
    tot
}

fn part_two(trees: &Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            max = i32::max(max, scenic_score(trees, row, col));
        }
    }
    max
}

fn scenic_score(trees: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut see_west = 0;
    let mut see_east = 0;
    let mut see_north = 0;
    let mut see_south = 0;
    let mut i: i32 = col as i32 - 1;
    let tree = trees[row][col];
    while i >= 0 {
        see_west += 1;
        if trees[row][i as usize] >= tree {
            break;
        }
        i -= 1;
    }
    i = col as i32 + 1;
    while i < trees[row].len() as i32 {
        see_east += 1;
        if trees[row][i as usize] >= tree {
            break;
        }
        i += 1;
    }
    i = row as i32 - 1;
    while i >= 0 {
        see_north += 1;
        if trees[i as usize][col] >= tree {
            break;
        }
        i -= 1;
    }
    i = row as i32 + 1;
    while i < trees.len() as i32 {
        see_south += 1;
        if trees[i as usize][col] >= tree {
            break;
        }
        i += 1;
    }
    see_west * see_east * see_north * see_south
}