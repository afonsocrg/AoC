use std::collections::HashSet;
use std::env;
use std::fs;

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

fn parse_input(input_file: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Failed to parse digit"))
                .collect()
        })
        .collect()
}

fn part_1(input: Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, &c)| {
                    return if c == 0 {
                        println!("Found trail head at ({}, {})", i, j);
                        let mut trail_ends: HashSet<(isize, isize)> = HashSet::new();
                        search_trail_end(&input, i as isize, j as isize, &mut trail_ends);
                        trail_ends.len() as u32
                    } else {
                        0
                    };
                })
                .sum::<u32>()
        })
        .sum()
}

fn part_2(input: Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, &c)| {
                    return if c == 0 {
                        count_paths(&input, i as isize, j as isize)
                    } else {
                        0
                    };
                })
                .sum::<u32>()
        })
        .sum()
}

fn search_trail_end(
    input: &Vec<Vec<u32>>,
    i: isize,
    j: isize,
    trail_ends: &mut HashSet<(isize, isize)>,
) {
    let current_height = input[i as usize][j as usize];

    if current_height == 9 {
        trail_ends.insert((i, j));
        return;
    }

    for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let ni: isize = (i + di).try_into().unwrap();
        let nj: isize = (j + dj).try_into().unwrap();

        if ni >= 0
            && nj >= 0
            && ni < input.len() as isize
            && nj < input[ni as usize].len() as isize
            && input[ni as usize][nj as usize] == input[i as usize][j as usize] + 1
        {
            search_trail_end(&input, ni, nj, trail_ends);
        }
    }
}

fn count_paths(input: &Vec<Vec<u32>>, i: isize, j: isize) -> u32 {
    let current_height = input[i as usize][j as usize];

    if current_height == 9 {
        return 1;
    }

    let mut count = 0;
    for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let ni: isize = (i + di).try_into().unwrap();
        let nj: isize = (j + dj).try_into().unwrap();

        if ni >= 0
            && nj >= 0
            && ni < input.len() as isize
            && nj < input[ni as usize].len() as isize
            && input[ni as usize][nj as usize] == input[i as usize][j as usize] + 1
        {
            count += count_paths(&input, ni, nj)
        }
    }

    count
}
