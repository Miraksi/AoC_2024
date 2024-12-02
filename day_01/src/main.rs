use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;


fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let reader = BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines().flatten() {
        let input: Vec<&str> = line.split("   ").collect();
        list1.push(input[0].parse::<u32>().unwrap());
        list2.push(input[1].parse::<u32>().unwrap());
    }
    part_one(&mut list1, &mut list2);
    part_two(&list1, &list2);
}

fn part_one(list1: &mut Vec<u32>, list2: &mut Vec<u32>) {
    list1.sort();
    list2.sort();

    let mut total_distance = 0;

    for (x, y) in list1.iter().zip(list2.iter()) {
        total_distance += x.abs_diff(*y);
    }
    println!("part one: {}", total_distance);
}

fn part_two(list1: &Vec<u32>, list2: &Vec<u32>) {
    let mut list2_set = HashMap::new();

    for x in list2.iter() {
        match list2_set.get(x) {
            None => list2_set.insert(*x, 1),
            Some(count) => list2_set.insert(*x, count + 1),
        };
    }    

    let mut sum = 0;
    for x in list1.iter() {
        if let Some(count) = list2_set.get(x) {
            sum += *x * *count;
        }
    }

    println!("part two: {}", sum);
}