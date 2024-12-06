use std::io::{BufReader, Read};
use std::fs::File;
use std::collections::{HashMap, HashSet};

fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let mut reader = BufReader::new(file);

    let mut tmp = String::new();
    reader.read_to_string(&mut tmp).expect("couldnt read to string");
    let input: Vec<&str> = tmp.split("\n\n").collect();

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in input[0].lines() {
        let values: Vec<&str> = line.split('|').collect();
        let x = values[0].parse::<u32>().expect("couldnt convert string to ");
        let y = values[1].parse::<u32>().expect("couldnt convert string to ");

        match rules.get_mut(&x) {
            Some(set) => {set.insert(y);},
            None => {rules.insert(x, HashSet::from([y]));},
        };
    }

    let updates: Vec<Vec<u32>> = input[1].lines()
        .map(
            |line| line.split(',').map(|x| x.parse::<u32>().expect("coudlnt convert to u32")).collect()
        ).collect();

    part_one(&rules, &updates);
}

fn part_one(rules: &HashMap<u32,HashSet<u32>>, updates: &Vec<Vec<u32>>) {
    let mut sum = 0;
    for update in updates.iter() {
        if is_valid(rules, update) {
            sum += update[update.len()/2];
        }
    }
    println!("part one: {sum}");
}

fn is_valid(rules: &HashMap<u32,HashSet<u32>>, update: &Vec<u32>) -> bool {
    for (i, x) in update.iter().enumerate() {
        if let Some(set) = rules.get(x) {
            for y in set.iter() {
                if update[..i].contains(y) {
                    return false;
                }
            }
        }
    }
    true
}