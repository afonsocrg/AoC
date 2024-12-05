use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

fn parse_input(input_file: &str) -> Vec<Vec<i32>> {
    let file = File::open(input_file).expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Could not parse number"))
                .collect()
        })
        .collect()
}

fn is_safe(v: &Vec<i32>) -> bool {
    let increasing = v.get(1).unwrap() > v.get(0).unwrap();
    for i in 1..v.len() {
        let diff = v.get(i).unwrap() - v.get(i - 1).unwrap();
        if !(1 <= diff.abs() && diff.abs() <= 3) || increasing != (diff > 0) {
            return false;
        }
    }
    true
}

fn part_1(input: Vec<Vec<i32>>) -> i32 {
    input.iter().filter(|level| is_safe(level)).count() as i32
}

fn part_2(input: Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .filter(|level| {
            if is_safe(level) {
                return true;
            } else {
                for i in 0..level.len() {
                    if is_safe(
                        &level
                            .iter()
                            .enumerate()
                            .filter(|(j, _)| *j != i)
                            .map(|(_, e)| *e)
                            .collect(),
                    ) {
                        return true;
                    }
                }
                false
            }
        })
        .count() as i32
}
