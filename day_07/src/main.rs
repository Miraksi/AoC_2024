use std::io::{BufRead, BufReader};
use std::fs::File;

struct Query {
    result: u128,
    numbers: Vec<u128>,
}
impl Query {
    fn new(line: &str) -> Self {
        let input: Vec<&str> = line.split(": ").collect();
        Self {
            result: input[0].parse().unwrap(),
            numbers: input[1].split(' ').map(|x| x.parse::<u128>().expect("input couldnt be parsed")).collect(),
        }
    }

    fn solvable_plus_mul(&self, index: usize, result: u128) -> bool {
        if result > self.result {
            return false;
        }
        if index >= self.numbers.len() {
            return result == self.result;
        }

        self.solvable_plus_mul(index + 1, result * self.numbers[index]) || self.solvable_plus_mul(index + 1, result + self.numbers[index])
    }

    fn solvable_advanced(&self, index: usize, result: u128) -> bool {
        if result > self.result {
            return false;
        }
        if index >= self.numbers.len() {
            return result == self.result;
        }
        
        let concat = (result.to_string() + &self.numbers[index].to_string()).parse().expect("faulty int concat");
        // println!("{} || {} = {}", result, self.numbers[index], concat);

        self.solvable_advanced(index + 1, result * self.numbers[index]) || 
        self.solvable_advanced(index + 1, result + self.numbers[index]) ||
        self.solvable_advanced(index + 1, concat)
    }
}

fn main() {
    let file = File::open("input.txt").expect("File couldnt be opened");
    let reader = BufReader::new(file);
    let mut queries = Vec::new();

    for line in reader.lines().flatten() {
        queries.push(Query::new(&line));
    }

    part_one(&queries);
    part_two(&queries);
}

fn part_one(queries: &Vec<Query>) {
    let mut sum = 0;

    for query in queries {
        if query.solvable_plus_mul(1, query.numbers[0]) {
            sum +=query.result;
        }
    }
    println!("part one: {sum}");
}

fn part_two(queries: &Vec<Query>) {
    let mut sum = 0;

    for query in queries {
        if query.solvable_advanced(1, query.numbers[0]) {
            sum +=query.result;
        }
    }
    println!("part two: {sum}");
}