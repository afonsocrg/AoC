use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

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

fn parse_input(input_file: &str) -> Vec<i32> {
    fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

fn part_1(input: Vec<i32>) -> i64 {
    input.iter().map(|n| {
        let mut n = *n;
        for _ in 0..2000 {
            n = next_number(n);
        }
        n as i64
    }).sum::<i64>()
}

fn part_2(input: Vec<i32>) -> i64 {
    let mut sequence_prices = HashMap::new();
    input.iter().for_each(|n| {
        let mut n = *n;
        let mut merchant_sequences = HashSet::new();
        let mut price = n % 10;
        let mut sequence: i32 = 0;

        // Initial setup
        for _ in 0..3 {
            let next = next_number(n);
            let price_next: i32 = next % 10;
            let diff = (price_next - price) & 0x1F;
            sequence = update_sequence(sequence, diff);
            n = next;
            price = price_next;
        }

        // Main loop: calculate sequence and optionally store price
        for _ in 3..2000 {
            let next = next_number(n);
            let price_next = next % 10;
            let diff = (price_next - price) & 0x1F;
            sequence = update_sequence(sequence, diff);
            if !merchant_sequences.contains(&sequence) {
                merchant_sequences.insert(sequence);
                *sequence_prices.entry(sequence).or_insert(0) += price_next;
            }
            n = next;
            price = price_next;
        }
    });

    *sequence_prices.values().max().unwrap() as i64
}

fn update_sequence(sequence: i32, diff: i32) -> i32 {
    (sequence << 5 | diff) & 0xFFFFF
}

fn next_number(mut n: i32) -> i32 {
    n = (n ^ (n << 6)) & 0xFFFFFF;
    n = (n ^ (n >> 5)) & 0xFFFFFF;
    n = (n ^ (n << 11)) & 0xFFFFFF;
    n
}