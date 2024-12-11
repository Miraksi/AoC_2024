use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashSet, HashMap};

use itertools::Itertools;

fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let reader = BufReader::new(file);
    let input: Vec<Vec<char>> = reader.lines().flatten().map(|line| line.chars().collect()).collect();

    let mut antennas_map = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '.' {
                continue;
            }
            antennas_map.entry(c).or_insert(Vec::new()).push((j as i32,i as i32)); // (x,y) as coordinates
        } 
    }

    part_one(&antennas_map, input[0].len() as i32, input.len() as i32);
    part_two(&antennas_map, input[0].len() as i32, input.len() as i32);
}

fn part_one(antennas_map: &HashMap<char,Vec<(i32,i32)>>, x_border: i32, y_border: i32) {
    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas_map.iter() {
        for combination in antennas.iter().combinations(2) {
            let fst = combination[0];
            let snd = combination[1];
            antinodes.insert((2*snd.0 - fst.0, 2*snd.1 - fst.1));
            antinodes.insert((2*fst.0 - snd.0, 2*fst.1 - snd.1));
        }
    }

    let mut count = 0;
    for antinode in antinodes.iter() {
        if in_bounds(antinode, x_border, y_border) {
            count += 1;
        }
    }

    println!("part_one: {count}");
}

fn part_two(antennas_map: &HashMap<char,Vec<(i32,i32)>>, x_border: i32, y_border: i32) {
    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas_map.iter() {
        for combination in antennas.iter().combinations(2) {
            let fst = combination[0];
            let snd = combination[1];

            let one_direction = (fst.0 - snd.0, fst.1 - snd.1); // my mind's tellin me noooo
            let mut current_antinode = *snd; 
            while in_bounds(&current_antinode, x_border, y_border) {
                antinodes.insert(current_antinode);
                current_antinode = (current_antinode.0 + one_direction.0, current_antinode.1 + one_direction.1);
            }

            let two_direction = (snd.0 - fst.0, snd.1 - fst.1); // BUT MY BOOOODDDYYYY
            current_antinode = *fst;
            while in_bounds(&current_antinode, x_border, y_border) {
                antinodes.insert(current_antinode);
                current_antinode = (current_antinode.0 + two_direction.0, current_antinode.1 + two_direction.1);
            }
        }
    }

    let mut count = 0;
    for antinode in antinodes.iter() {
        if in_bounds(antinode, x_border, y_border) {
            count += 1;
        }
    }
    println!("part_two: {count}");
}

fn in_bounds(antinode: &(i32,i32), x_border: i32, y_border: i32) -> bool {
    if antinode.0 < 0 || antinode.0 >= x_border {
        return false;
    }
    if antinode.1 < 0 || antinode.1 >= y_border {
        return false;
    }
    true
}