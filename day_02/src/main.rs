use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

enum Direction {
    Ascending,
    Descending,
} 

fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let reader = BufReader::new(file);

    let mut levels = Vec::new();

    for line in reader.lines().flatten() {
        let level: Vec<u32> = line.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
        levels.push(level);
    }
    part_one(&levels);
    part_two(&levels);
}

fn part_one(levels: &Vec<Vec<u32>>) {
    let mut safe_count = 0;
    for level in levels.iter() {
        if is_safe(level) {
            safe_count += 1;
        }
    }
    println!("part one: {}", safe_count);
}

fn part_two(levels: &Vec<Vec<u32>>) {
    let mut safe_count = 0;
    for level in levels.iter() {
        for i in 0..level.len() {
            let mut level_slice = Vec::new();
            for x in level[..i].iter().chain(level[i+1..].iter()) {
                level_slice.push(*x);
            }

            if is_safe(&level_slice) {
                safe_count += 1;
                break;
            }
        }
    }
    println!("part one: {}", safe_count);
}

fn is_safe(level: &Vec<u32>) -> bool {
    let mut direction = None;
    
    for window in level.windows(2) {
        let x = window[0];
        let y = window[1];

        match direction {
            None => {
                if x < y {
                    direction = Some(Direction::Ascending);
                }
                else if x > y {
                    direction = Some(Direction::Descending);
                }
                else {
                    return false;
                }
            },
            Some(Direction::Ascending) => {
                if x >= y {
                    return false;
                }
            },
            Some(Direction::Descending) => {
                if x <= y {
                    return false;
                }
            },
        }

        let diff = x.abs_diff(y);
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}