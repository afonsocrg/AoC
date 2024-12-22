use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq)]
struct State(char, String, usize);

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

fn parse_input(input_file: &str) -> Vec<String> {
    fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn part_1(input: Vec<String>) -> i64 {
    solve(input, 3)
}

fn part_2(input: Vec<String>) -> i64 {
    solve(input, 26)
}

fn solve(input: Vec<String>, depth: usize) -> i64 {
    let mut cache: HashMap<State, i64> = HashMap::new();
    input
        .iter()
        .map(|l| {
            // Remove last character and parse as number
            let number = l[..l.len() - 1].parse::<i64>().unwrap();

            number * count_min_button_presses('A', &l, depth, true, &mut cache)
        })
        .sum()
}

fn count_min_button_presses(
    from: char,
    sequence: &String,
    depth: usize,
    is_numeric: bool,
    cache: &mut HashMap<State, i64>,
) -> i64 {
    if depth == 0 {
        return sequence.len() as i64;
    }

    if let Some(cached) = cache.get(&State(from, sequence.clone(), depth)) {
        return *cached;
    }

    let mut current = 'A';
    let mut cost = 0;
    for next in sequence.chars() {
        let options = if is_numeric {
            get_numeric_path_combinations(current, next)
        } else {
            get_directional_path_combinations(current, next)
        };

        let mut min_child_cost = None;
        for option in options {
            let cost = if depth > 1 {
                count_min_button_presses('A', &option, depth - 1, false, cache)
            } else {
                option.len() as i64
            };
            if min_child_cost.is_none() || cost < min_child_cost.unwrap() {
                min_child_cost = Some(cost);
            }
        }
        let min_child_cost = min_child_cost.unwrap();
        cost += min_child_cost;
        current = next;
    }

    cache.insert(State(from, sequence.clone(), depth), cost);

    cost
}

fn get_numeric_path_combinations(from: char, to: char) -> HashSet<String> {
    let mut results = HashSet::new();

    if from == to {
        results.insert("A".to_string());
        return results;
    }

    let (fx, fy) = get_numeric_keypad_coordinate(from);
    let (tx, ty) = get_numeric_keypad_coordinate(to);
    let (dx, dy) = (tx - fx, ty - fy);

    // We are just considering the combinations that have the least amount of turns (at most 1 turn)
    let horizontal = (if dx > 0 { ">" } else { "<" }).repeat(dx.abs().try_into().unwrap());
    let vertical = (if dy > 0 { "^" } else { "v" }).repeat(dy.abs().try_into().unwrap());

    // Make sure there is no path going through (0, 0)
    if !(fy == 0 && tx == 0) {
        results.insert(horizontal.clone() + &vertical + "A");
    }
    if !(fx == 0 && ty == 0) {
        results.insert(vertical.clone() + &horizontal + "A");
    }

    results
}

fn get_directional_path_combinations(from: char, to: char) -> HashSet<String> {
    let mut results = HashSet::new();
    if from == to {
        results.insert("A".to_string());
        return results;
    }
    let (fx, fy) = get_directional_keypad_coordinate(from);
    let (tx, ty) = get_directional_keypad_coordinate(to);
    let (dx, dy) = (tx - fx, ty - fy);
    let horizontal = (if dx > 0 { ">" } else { "<" }).repeat(dx.abs().try_into().unwrap());
    let vertical = (if dy > 0 { "^" } else { "v" }).repeat(dy.abs().try_into().unwrap());

    // Make sure there is no path going through (0, 1)
    if !(fx == 0 && fy == 0) {
        results.insert(vertical.clone() + &horizontal + "A");
    }

    if !(tx == 0 && ty == 0) {
        results.insert(horizontal.clone() + &vertical + "A");
    }
    results
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
fn get_numeric_keypad_coordinate(c: char) -> (i64, i64) {
    match c {
        '0' => (1, 0),
        'A' => (2, 0),
        '1' => (0, 1),
        '2' => (1, 1),
        '3' => (2, 1),
        '4' => (0, 2),
        '5' => (1, 2),
        '6' => (2, 2),
        '7' => (0, 3),
        '8' => (1, 3),
        '9' => (2, 3),
        _ => panic!("Invalid character"),
    }
}

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
fn get_directional_keypad_coordinate(c: char) -> (i64, i64) {
    match c {
        '<' => (0, 0),
        'v' => (1, 0),
        '>' => (2, 0),
        '^' => (1, 1),
        'A' => (2, 1),
        _ => panic!("{}", format!("Invalid character: {}", c)),
    }
}
