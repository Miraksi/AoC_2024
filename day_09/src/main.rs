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

    // part_one(memory_map.clone());
    part_two(memory_map.clone());
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
    println!("part one: {}", sum);
}

fn part_two(mut memory_map: Vec<Option<u128>>) {
    let mut end = memory_map.len() - 1;

    while end >= 1 {

        if memory_map[end].is_none() {
            end -= 1;
            continue;
        }
        let file = memory_map[end].unwrap();
        let start = memory_map.iter().position(
            |&x| match x {
                Some(index) => index == file,
                None => false,
            } 
        ).unwrap();
        // println!("{}: {}, {}", file, start, end);
        let file_size = (end - start) + 1;
        if let Some(mem_start) = find_memory(&memory_map, file_size) {
            if mem_start < end {
                for space in memory_map[mem_start..mem_start+file_size].iter_mut() {
                    *space = Some(file);
                }
                for file in memory_map[start..=end].iter_mut() {
                    *file = None;
                }
            }
        }
        if start == 0 {
            break;
        }
        end = start - 1;
    }
    let mut sum = 0;
    for (i, &file) in memory_map.iter().enumerate() {
        if let Some(x) = file {
            sum += i as u128 * x;
        }
    }
    // println!("{:?}", memory_map);
    println!("part two: {}", sum);
}

fn find_memory(memory_map: &Vec<Option<u128>>, size: usize) -> Option<usize> {
    'outer: for (index, window) in memory_map.windows(size).enumerate() {
        for x in window.iter() {
            if x.is_some() {
                continue 'outer;
            }
        }
        return Some(index);
    }
    None
}