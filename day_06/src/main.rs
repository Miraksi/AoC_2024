use std::io::{BufReader, Read};
use std::fs::File;

fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let mut reader = BufReader::new(file);

    let mut tmp = String::new();
    reader.read_to_string(&mut tmp).expect("couldnt read to string");

    let input: Vec<Vec<char>> = tmp.lines().map(|line| line.chars().collect()).collect();    

    part_one(&input);
}

fn part_one(input: &Vec<Vec<char>>) {
    let x_border = input[0].len() as i32 - 1;
    let y_border = input.len() as i32 - 1;
    let mut pos = get_starting_pos(&input).expect("no starting position found");
    let mut dir = (-1, 0);
    let mut visited = vec![vec![false; x_border as usize + 1]; y_border as usize + 1];
    let mut count = 0;

    while in_bounds(x_border, y_border, pos) {
        if !visited[pos.0 as usize][pos.1 as usize] {
            count += 1;
        }
        visited[pos.0 as usize][pos.1 as usize] = true;
        make_move(input, &mut pos, &mut dir);
    }
    println!("part one: {count}");
}

fn get_starting_pos(input: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}

fn in_bounds(x_border: i32, y_border: i32, pos: (i32, i32)) -> bool {
    if pos.0 < 0 || pos.0 > x_border {
        return false;
    }
    if pos.1 < 0 || pos.1 > y_border {
        return false;
    }
    true
}

fn make_move(input: &Vec<Vec<char>>, pos: &mut (i32, i32), dir: &mut (i32, i32)) {
    let x_border = input[0].len() as i32 - 1;
    let y_border = input.len() as i32 - 1;
    let mut next_pos = (pos.0 + dir.0, pos.1 + dir.1);

    if !in_bounds(x_border, y_border, next_pos) {
        *pos = next_pos;
        return;
    }

    while input[next_pos.0 as usize][next_pos.1 as usize] == '#' {
        // Rotation
        *dir = (dir.1, dir.0 * (-1));
        next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    *pos = next_pos;
}