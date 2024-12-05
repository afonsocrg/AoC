use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

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

fn parse_input(input_file: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    // let file = File::open(input_file).expect("File not found");
    let content = fs::read_to_string(input_file).expect("Could not read file");
    let (part_1, part_2) = content
        .split_once("\n\n")
        .expect("Failed to split with blank line");

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    part_1.lines().for_each(|line| {
        let numbers: Vec<i32> = line
            .split('|')
            .map(|part| part.parse::<i32>().expect("Failed to parse rule"))
            .collect();
        let a = numbers.get(0).expect("Failed to get a");
        let b = numbers.get(1).expect("Failed to get b");
        if !rules.contains_key(a) {
            rules.insert(*a, HashSet::new());
        }
        rules.get_mut(a).unwrap().insert(*b);
    });

    let updates: Vec<Vec<i32>> = part_2
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().expect("Could not parse number"))
                .collect()
        })
        .collect();

    (rules, updates)
}

// Returns whether a must precede b according to the given rules
fn must_precede(a: i32, b: i32, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    match rules.get(&a) {
        Some(s) => s.contains(&b),
        _ => false,
    }
}

fn is_valid(rules: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    for i in 1..update.len() {
        for j in 0..i {
            if must_precede(update[i], update[j], rules) {
                return false;
            }
        }
    }
    true
}

fn part_1((rules, updates): (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>)) -> i32 {
    updates
        .into_iter()
        .filter(|u| is_valid(&rules, u))
        .map(|u| u.get(u.len() / 2).unwrap().clone())
        .sum()
}

fn part_2((rules, updates): (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>)) -> i32 {
    updates
        .into_iter()
        .filter(|u| !is_valid(&rules, u))
        .map(|mut u| {
            let mut i = 1;
            while i < u.len() {
                let mut j = 0;
                while j < i {
                    if must_precede(u[i], u[j], &rules) {
                        u.swap(i, j);
                        i = j + 1;
                        break;
                    }
                    j += 1;
                }
                i += 1;
            }
            u
        })
        .map(|u| u.get(u.len() / 2).unwrap().clone())
        .sum()
}
