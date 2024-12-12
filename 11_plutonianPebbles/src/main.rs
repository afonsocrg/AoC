use std::collections::HashMap;
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

fn parse_input(input_file: &str) -> Vec<u64> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse digit"))
        .collect()
}

fn part_1(input: Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|n| expand_number(*n, 25, &mut cache))
        .sum()
}

fn part_2(input: Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|n| expand_number(*n, 75, &mut cache))
        .sum()
}

fn expand_number(n: u64, height: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if height == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&(n, height)) {
        return *cached;
    }

    let result = if height == 0 {
        1
    } else {
        if n == 0 {
            expand_number(1, height - 1, cache)
        } else {
            let n_digits = get_n_digits(n);
            if n_digits % 2 == 0 {
                let div = u64::pow(10, (n_digits / 2).try_into().unwrap());
                let left = n / div;
                let right = n % div;
                expand_number(left, height - 1, cache)
                    + expand_number(right, height - 1, cache)
            } else {
                expand_number(n * 2024, height - 1, cache)
            }
        }
    };
    cache.insert((n, height), result);
    result
}

fn get_n_digits(mut n: u64) -> u8 {
    let mut n_digits = 0;
    while n >= 1 {
        n /= 10;
        n_digits += 1;
    }
    n_digits
}

// // This was fast enough for part 1, but not for part 2...
// // We need memoization
// fn expand_input(input: &mut LinkedList<u32>, n_iterations: usize) -> usize {
//     for i in 0..n_iterations {
//         let mut cursor = input.cursor_front_mut();
//         while let Some(item) = cursor.current() {
//             if *item == 0 {
//                 *item = 1;
//             } else {
//                 let n_digits = get_n_digits(*item);
//                 if n_digits % 2 == 0 {
//                     let div = u32::pow(10, n_digits / 2);
//                     let left = *item / div;
//                     let right = *item % div;
//                     cursor.insert_before(left);
//                     *cursor.current().unwrap() = right;
//                 } else {
//                     *cursor.current().unwrap() = *item * 2024;
//                 }
//             }
//             cursor.move_next();
//         }
//     }
//     input.len()
// }
