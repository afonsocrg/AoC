use std::collections::{HashMap, HashSet};
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
    println!("Answer: {}", result);
    println!("Executed in {:?}", elapsed);
}

fn parse_input(input_file: &str) -> Vec<(String, String)> {
    fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

fn part_1(input: Vec<(String, String)>) -> String {
    let mut adjacencies: HashMap<String, HashSet<String>> = HashMap::new();
    let mut trios: HashSet<(String, String, String)> = HashSet::new();

    for (a, b) in input {
        adjacencies
            .entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        adjacencies
            .entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());

        let intersection: Vec<_> = adjacencies[&a]
            .intersection(&adjacencies[&b])
            .cloned()
            .collect();
        if !intersection.is_empty() {
            for c in intersection {
                if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
                    let (a, b, c) = sort(&a, &b, &c);
                    trios.insert((a.clone(), b.clone(), c.clone()));
                }
            }
        }
    }
    trios.len().to_string()
}

fn part_2(input: Vec<(String, String)>) -> String {
    let mut adjacencies: HashMap<String, HashSet<String>> = HashMap::new();
    let mut largest_clique = HashSet::new();
    for (a, b) in input {
        adjacencies
            .entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        adjacencies
            .entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());

        let intersection: Vec<_> = adjacencies[&a]
            .intersection(&adjacencies[&b])
            .cloned()
            .collect();
        if !intersection.is_empty() {
            let mut clique = HashSet::from([a, b]);
            for c in intersection {
                if clique.len() == clique.intersection(&adjacencies[&c]).count() {
                    clique.insert(c.clone());
                }
            }
            if clique.len() > largest_clique.len() {
                largest_clique = clique;
            }
        }
    }
    let mut sorted_clique = largest_clique.iter().collect::<Vec<_>>();
    sorted_clique.sort();
    sorted_clique
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn sort<T: PartialOrd>(a: T, b: T, c: T) -> (T, T, T) {
    if a < b {
        if b < c {
            (a, b, c)
        } else if a < c {
            (a, c, b)
        } else {
            (c, a, b)
        }
    } else {
        if a < c {
            (b, a, c)
        } else if b < c {
            (b, c, a)
        } else {
            (c, b, a)
        }
    }
}
