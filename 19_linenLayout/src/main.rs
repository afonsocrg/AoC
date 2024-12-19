use std::collections::{HashSet, HashMap};
use std::cmp;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file is required");
    let input = parse_input(input_file);

    let now = Instant::now();
    let result = if args.contains(&"--part-2".to_string()) {
        part_2(input)
    } else {
        part_1(input)
    };
    let elapsed = now.elapsed();
    println!("{:?}", result);
    println!("Executed in {:?}", elapsed);
}

fn parse_input(input_file: &str) -> (HashSet<String>, Vec<String>) {
    let input = fs::read_to_string(input_file)
        .expect("Failed to read file");
    let (patterns, designs) = input.split_once("\n\n")
        .expect("Failed to split input");

    let patterns: HashSet<String> = patterns.split(", ").map(|s| s.to_string()).collect();
    let designs: Vec<String> = designs.split("\n").map(|s| s.to_string()).collect();

    (patterns, designs)
}

fn part_1((patterns, designs): (HashSet<String>, Vec<String>)) -> i64 {
    let max_len = patterns.iter().map(|p| p.len()).max().unwrap();
    let mut memoized = HashMap::new();
    designs.iter().filter(|design| {
        can_create_design(&patterns, &design[..], max_len, &mut memoized)
    }
    ).count() as i64
}

fn part_2((patterns, designs): (HashSet<String>, Vec<String>)) -> i64 {
    let max_len = patterns.iter().map(|p| p.len()).max().unwrap();
    let mut memoized = HashMap::new();
    designs.iter().map(|design| {
        how_many_ways_can_create_design(&patterns, &design[..], max_len, &mut memoized)
    }
    ).sum()
}

// This function is slightly faster because we just need to say if it's possible or not
// Keeping it separate
fn can_create_design(patterns: &HashSet<String>, design: &str, max_len: usize, memoized: &mut HashMap<String, bool>) -> bool {
    if let Some(result) = memoized.get(design) {
        return *result;
    }

    if design.len() == 0 {
        memoized.insert(design.to_string(), true);
        return true;
    }

    for l in 1..=cmp::min(max_len, design.len()) {
        if patterns.contains(&design[..l]) {
            if can_create_design(patterns, &design[l..], max_len, memoized) {
                memoized.insert(design.to_string(), true);
                return true;
            }
        }
    }

    memoized.insert(design.to_string(), false);
    false
}

fn how_many_ways_can_create_design(patterns: &HashSet<String>, design: &str, max_len: usize, memoized: &mut HashMap<String, i64>) -> i64 {
    if let Some(result) = memoized.get(design) {
        return *result;
    }

    if design.len() == 0 {
        memoized.insert(design.to_string(), 1);
        return 1;
    }

    let mut count = 0;
    for l in 1..=cmp::min(max_len, design.len()) {
        if patterns.contains(&design[..l]) {
            count += how_many_ways_can_create_design(patterns, &design[l..], max_len, memoized);
        }
    }

    memoized.insert(design.to_string(), count);
    count
}