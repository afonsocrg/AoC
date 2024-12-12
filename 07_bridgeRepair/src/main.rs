use std::env;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Equation {
    result: u64,
    right: Vec<u64>,
}

enum Op {
    Concat,
    Mul,
    Add,
}

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

fn parse_input(input_file: &str) -> Vec<Equation> {
    fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.split(": ").collect();
            let result = split
                .get(0)
                .expect("Failed to get equation result")
                .parse()
                .expect("Failed to parse equation result");
            let right = split
                .get(1)
                .expect("Failed to get equation numbers")
                .split(" ")
                .map(|s| s.parse().expect("Failed to parse equation number"))
                .collect();
            Equation { result, right }
        })
        .collect()
}

fn part_1(input: Vec<Equation>) -> u64 {
    input
        .iter()
        .filter(|e| is_solvable(e.result, e.right[0], &e.right[1..], false))
        .map(|e| e.result)
        .sum()
}

fn part_2(input: Vec<Equation>) -> u64 {
    input
        .iter()
        .filter(|e| is_solvable(e.result, e.right[0], &e.right[1..], true))
        .map(|e| e.result)
        .sum()
}

fn is_solvable(target: u64, current: u64, numbers: &[u64], try_concat: bool) -> bool {
    if current > target {
        return false;
    }

    if numbers.len() == 0 {
        return current == target;
    }

    let n = numbers.get(0).expect("Failed to get number");

    let ops: Vec<Op> = if try_concat {
        vec![Op::Concat, Op::Mul, Op::Add]
    } else {
        vec![Op::Mul, Op::Add]
    };

    for op in ops {
        let new_curr = match op {
            Op::Concat => (current * 10_u64.pow(n.to_string().len() as u32)) + n,
            Op::Mul => current * n,
            Op::Add => current + n,
        };

        if new_curr <= target && is_solvable(target, new_curr, &numbers[1..], try_concat) {
            return true;
        }
    }

    false
}
