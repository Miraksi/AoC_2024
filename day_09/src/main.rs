use std::io::{Read, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let mut reader = BufReader::new(file);

    let mut tmp = String::new();
    reader.read_to_string(&mut tmp).expect("couldnt read to string");

    let input: Vec<u128> = tmp.chars().map(|c| c.to_string().parse().expect("couldnt parse character to digit")).collect(); 

    let mut memory_map = Vec::new();
    for (index, &amount) in input.iter().enumerate() {
        let file;
        if index % 2 == 0 {
            file = Some(index as u128 /2);
        }
        else {
            file = None;
        }
        memory_map.append(&mut vec![file; amount as usize]);
    }

    part_one(memory_map.clone());
}

fn part_one(mut memory_map: Vec<Option<u128>>) {
    let mut first_space = memory_map.iter().position(|&x| x.is_none()).unwrap();
    let mut last_file = memory_map.len() - 1 - memory_map.iter().rev().position(|&x| x.is_some()).unwrap();

    while first_space <= last_file {
        memory_map[first_space] = memory_map[last_file];
        memory_map[last_file] = None;
        first_space = memory_map.iter().position(|&x| x.is_none()).unwrap();
        last_file = memory_map.len() - 1 - memory_map.iter().rev().position(|&x| x.is_some()).unwrap();
    }

    let mut sum = 0;
    let mut index = 0;
    let mut iter = memory_map.iter();
    while let Some(Some(x)) = iter.next() {
        sum += index * x;
        index += 1; 
    }
    println!("{:?}", sum);
}