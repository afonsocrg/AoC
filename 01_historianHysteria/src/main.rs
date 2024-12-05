use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::iter::zip;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file is required");
    let input = parse_input(input_file);
    if args.get(2).is_some() && args.get(2).unwrap() == "2" {
        println!("{:?}", part_2(input));
    } else {
        println!("{:?}", part_1(input));
    }
}

fn parse_input(input_file: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(input_file).expect("File not found");
    let reader = std::io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .expect("Could not read line")
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            (numbers[0], numbers[1])
        })
        .unzip()
}

fn part_1(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let (mut col1, mut col2) = input;
    col1.sort();
    col2.sort();
    zip(col1, col2).map(|(a, b)| (a - b).abs()).sum()
}

fn part_2(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let (col1, col2) = input;
    let mut histogram = HashMap::new();
    col2.iter().for_each(|n| {
        histogram
            .entry(n)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });

    col1.iter()
        .map(|n| n * histogram.get(n).unwrap_or(&0))
        .sum()
}
