use std::{io::{BufReader, BufRead}, fs::File};

pub fn print_solution() {
    let input = File::open("inputs/input2.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let score1 = calc_score_1(&lines);
    println!("Day 2 Part 1: {}", score1);
    let score2 = calc_score_2(&lines);
    println!("Day 2 Part 2: {}", score2);
}

fn calc_score_1(lines: &Vec<String>) -> i32 {
    let mut score = 0;
    for line in lines {
        if line != "" {
            let line: Vec<char> = line.chars().collect();
            let opp = line[0];
            let me = line[2];
            score += rock_paper_scissors(opp, me);
        }
    }
    score
}

fn calc_score_2(lines: &Vec<String>) -> i32 {
    let mut score = 0;
    for line in lines {
        if line != "" {
            let line: Vec<char> = line.chars().collect();
            let opp = line[0];
            let goal = line[2];
            let me = match opp {
                'A' => match goal {
                    'X' => 'Z',
                    'Y' => 'X',
                    'Z' => 'Y',
                    _ => ' '
                }
                'B' => goal,
                'C' => match goal {
                    'X' => 'Y',
                    'Y' => 'Z',
                    'Z' => 'X',
                    _ => ' '
                }
                _ => ' '
            };
            score += rock_paper_scissors(opp, me);
        }
    }
    score
}

fn rock_paper_scissors(opp: char, me: char) -> i32 {
    match opp {
        'A' => match me {
            'X' => 4,
            'Y' => 8,
            'Z' => 3,
            _ => 0
        }
        'B' => match me {
            'X' => 1,
            'Y' => 5,
            'Z' => 9,
            _ => 0
        }
        'C' => match me {
            'X' => 7,
            'Y' => 2,
            'Z' => 6,
            _ => 0
        }
        _ => 0
    }
}