use std::env;
use std::fs;

#[derive(Debug)]
struct Equation {
    result: u64,
    right: Vec<u64>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file is required");
    let input = parse_input(input_file);

    if args.contains(&"--part-2".to_string()) {
        println!("{:?}", part_2(input));
    } else {
        println!("{:?}", part_1(input));
    }
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
        .filter(|e| can_be_solved(e))
        .map(|e| e.result)
        .sum()
}

fn part_2(input: Vec<Equation>) -> u64 {
    input
        .iter()
        .filter(|e| can_be_solved_2(e))
        .map(|e| e.result)
        .sum()
}

#[derive(Debug, PartialEq)]
enum SearchStatus {
    Correct,
    Incorrect,
    TooHigh,
    TooLow, // Used only when no
}

fn is_solvable_recursive(target: u64, current: u64, numbers: &[u64], depth: usize) -> SearchStatus {
    // print_indent(depth);
    // println!(
    //     "Target: {:?}, Current: {:?}, Numbers: {:?}",
    //     target, current, numbers
    // );

    if current > target {
        // print_indent(depth);
        // println!("Current is too high");
        return SearchStatus::TooHigh;
    }

    if numbers.len() == 0 {
        // print_indent(depth);
        // println!("No numbers left");

        let result = if current == target {
            SearchStatus::Correct
        } else if current < target {
            // We may not need this case, because we're checking
            // if current > target above
            SearchStatus::TooLow
        } else {
            SearchStatus::TooHigh
        };

        // print_indent(depth);
        // println!("Result: {:?}", result);
        return result;
    }

    let n = numbers.get(0).expect("Failed to get number");
    let new_current_mul = current * n;
    // print_indent(depth);
    // println!("{} * {} = {}", current, n, new_current_mul);

    if new_current_mul > target {
        // print_indent(depth);
        // println!("Value is too high...");
    } else {
        let mul_result = is_solvable_recursive(target, new_current_mul, &numbers[1..], depth + 1);

        // print_indent(depth);
        // println!("Got result from search: {:?}", mul_result);

        if mul_result == SearchStatus::Correct {
            // if mul_result == SearchStatus::Correct || mul_result == SearchStatus::TooLow {
            // print_indent(depth);
            // println!("Returning result...");
            return mul_result;
        }
    }

    // print_indent(depth);
    // println!("Trying addition...");
    let new_current_add = current + n;
    // print_indent(depth);
    // println!("{} + {} = {}", current, n, new_current_add);

    let add_result = is_solvable_recursive(target, new_current_add, &numbers[1..], depth + 1);

    // print_indent(depth);
    // println!("Add result: {:?}", add_result);

    if add_result == SearchStatus::Correct {
        // if add_result == SearchStatus::Correct || add_result == SearchStatus::TooHigh {
        return add_result;
    }

    SearchStatus::Incorrect
}

fn is_solvable_recursive_2(
    target: u64,
    current: u64,
    numbers: &[u64],
    depth: usize,
) -> SearchStatus {
    // print_indent(depth);
    // println!(
    //     "Target: {:?}, Current: {:?}, Numbers: {:?}",
    //     target, current, numbers
    // );

    if current > target {
        // print_indent(depth);
        // println!("Current is too high");
        return SearchStatus::TooHigh;
    }

    if numbers.len() == 0 {
        // print_indent(depth);
        // println!("No numbers left");

        let result = if current == target {
            SearchStatus::Correct
        } else if current < target {
            // We may not need this case, because we're checking
            // if current > target above
            SearchStatus::TooLow
        } else {
            SearchStatus::TooHigh
        };

        // print_indent(depth);
        // println!("Result: {:?}", result);
        return result;
    }

    let n = numbers.get(0).expect("Failed to get number");

    // Try concatenating
    // print_indent(depth);
    // println!("Trying concatenation...");
    let new_current_concat = concatenate_numbers(current, *n);
    // print_indent(depth);
    // println!("{} || {} = {}", current, n, new_current_concat);
    if new_current_concat > target {
        // print_indent(depth);
        // println!("Concatenation is too high...");
    } else {
        let concat_result =
            is_solvable_recursive_2(target, new_current_concat, &numbers[1..], depth + 1);
        if concat_result == SearchStatus::Correct {
            // if concat_result == SearchStatus::Correct || concat_result == SearchStatus::TooLow {
            // print_indent(depth);
            // println!("Returning result...");
            return concat_result;
        }
    }

    // Try multiplying
    // print_indent(depth);
    // println!("Trying multiplication...");
    let new_current_mul = current * n;
    // print_indent(depth);
    // println!("{} * {} = {}", current, n, new_current_mul);

    if new_current_mul > target {
        // print_indent(depth);
        // println!("Value is too high...");
    } else {
        let mul_result = is_solvable_recursive_2(target, new_current_mul, &numbers[1..], depth + 1);

        // print_indent(depth);
        // println!("Got result from search: {:?}", mul_result);

        if mul_result == SearchStatus::Correct {
            // if mul_result == SearchStatus::Correct || mul_result == SearchStatus::TooLow {
            // print_indent(depth);
            // println!("Returning result...");
            return mul_result;
        }
    }

    // print_indent(depth);
    // println!("Trying addition...");
    let new_current_add = current + n;
    // print_indent(depth);
    // println!("{} + {} = {}", current, n, new_current_add);

    let add_result = is_solvable_recursive_2(target, new_current_add, &numbers[1..], depth + 1);

    // print_indent(depth);
    // println!("Add result: {:?}", add_result);

    if add_result == SearchStatus::Correct {
        // if add_result == SearchStatus::Correct || add_result == SearchStatus::TooHigh {
        return add_result;
    }

    SearchStatus::Incorrect
}

fn concatenate_numbers(n1: u64, n2: u64) -> u64 {
    (n1 * 10_u64.pow(n2.to_string().len() as u32)) + n2
}

fn can_be_solved(equation: &Equation) -> bool {
    // println!("=============== Testing equation ===============");
    // println!("{:?}: {:?}", equation.result, equation.right);
    let result = is_solvable_recursive(equation.result, equation.right[0], &equation.right[1..], 1)
        == SearchStatus::Correct;
    // println!(">>>>>>>>>>>>: {:?}", result);
    result
}

fn can_be_solved_2(equation: &Equation) -> bool {
    // println!("=============== Testing equation ===============");
    // println!("{:?}: {:?}", equation.result, equation.right);
    let result =
        is_solvable_recursive_2(equation.result, equation.right[0], &equation.right[1..], 1)
            == SearchStatus::Correct;
    // println!(">>>>>>>>>>>>: {:?}", result);
    result
}

fn print_indent(depth: usize) {
    print!("{}", "\t".repeat(depth));
}
