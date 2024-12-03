use std::io::{BufReader, Read};
use std::fs::File;
use regex::Regex;

fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input).expect("couldnt read to string");

    part_one(&input);
}

fn part_one(input: &str) {
    let re = Regex::new(r"(mul\([0-9]+,[0-9]+\))").expect("not valid regex");
    let found_matches = re.find_iter(input).map(|m| m.as_str());

    let mut sum = 0;
    for found_match in found_matches {
        let mut prod = 1;
        for number in found_match[4..found_match.len()-1].split(',') {
            prod *= number.parse::<u32>().unwrap();
        }
        sum += prod;
    }

    println!("part one: {sum}");
}