use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let reader = BufReader::new(file);

    let mut input = Vec::new();

    for line in reader.lines().flatten() {
        input.push(line.chars().collect());
    }

    part_one(&input);
    part_two(&input);
}


fn part_one(input: &Vec<Vec<char>>) {
    let mut count = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'X' {
                count += find_xmas(input, i,j,1,0);
                count += find_xmas(input, i,j,-1,0);
                count += find_xmas(input, i,j,1,1);
                count += find_xmas(input, i,j,-1,1);
                count += find_xmas(input, i,j,1,-1);
                count += find_xmas(input, i,j,-1,-1);
                count += find_xmas(input, i,j,0,1);
                count += find_xmas(input, i,j,0,-1);
            }
        }
    }
    println!("part one: {}", count);
}

fn find_xmas(input: &Vec<Vec<char>>, mut i: usize, mut j: usize, x_dir: i16, y_dir: i16) -> u32 {
    if x_dir < 0 && i < 3 {
        return 0;
    }
    if x_dir > 0 && i > input[0].len()-4 {
        return 0;
    }
    if y_dir < 0 && j < 3 {
        return 0;
    }
    if y_dir > 0 && j > input.len()-4 {
        return 0;
    }

    let xmas = "XMAS";
    for c in xmas.chars() {
        if c != input[i][j] {
            return 0;
        }
        i = ((i as i16) + x_dir) as usize;
        j = ((j as i16) + y_dir) as usize;
    }
    1
}

fn part_two(input: &Vec<Vec<char>>) {
    let mut count = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != 'A' {
                continue;
            }
            if is_cross_mas(input, i, j) {
                count += 1;
            }
        }
    }
    println!("part one: {}", count);
}

fn is_cross_mas(input: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 1 || i > input[0].len() - 2 {
        return false;
    }
    if j < 1 || j > input.len() - 2 {
        return false;
    }
    let first_diag = input[i-1][j-1] == 'M' && input[i+1][j+1] == 'S' || input[i-1][j-1] == 'S' && input[i+1][j+1] == 'M';
    let second_diag = input[i-1][j+1] == 'M' && input[i+1][j-1] == 'S' || input[i-1][j+1] == 'S' && input[i+1][j-1] == 'M';

    first_diag && second_diag
}