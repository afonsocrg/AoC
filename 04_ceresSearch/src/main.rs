use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const TARGET_WORD: &str = "XMAS";

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

fn parse_input(input_file: &str) -> Vec<Vec<char>> {
    let file = File::open(input_file).expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Could not read line").chars().collect())
        .collect()
}

fn crossword_search(input: &Vec<Vec<char>>, word: &str, i: isize, j: isize) -> i32 {
    let word_len: isize = word.len().try_into().expect("Word length is too large");
    let word_chars = word.chars().collect::<Vec<char>>();

    let mut count = 0;

    // Assuming every row has the same length
    let n_rows = input.len() as isize;
    let n_cols = input[0].len() as isize;

    for di in -1..=1 as isize {
        let last_i = i + (di * (word_len - 1));
        if last_i < 0 || last_i >= n_rows {
            continue;
        }
        for dj in -1..=1 as isize {
            if di == 0 && dj == 0 {
                continue;
            }
            let last_j = j + (dj * (word_len - 1));
            if last_j < 0 || last_j >= n_cols {
                continue;
            }

            let mut k: isize = 0;
            while k < word_len {
                let ii = (i + (di * k)) as usize;
                let jj = (j + (dj * k)) as usize;
                let c = input[ii][jj];
                let w = word_chars[k as usize];
                if c != w {
                    break;
                }
                k += 1;
            }
            if k == word_len {
                count += 1;
            }
        }
    }
    count
}

fn part_1(input: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            count += crossword_search(&input, TARGET_WORD, i as isize, j as isize);
        }
    }
    count
}

fn is_x_mas(
    center: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
) -> bool {
    center == 'A'
        && ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M'))
        && ((top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M'))
}

fn part_2(input: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 1..input.len() - 1 {
        for j in 1..input[i].len() - 1 {
            let center = input[i][j];
            let top_left = input[i - 1][j - 1];
            let top_right = input[i - 1][j + 1];
            let bottom_left = input[i + 1][j - 1];
            let bottom_right = input[i + 1][j + 1];
            if is_x_mas(center, top_left, top_right, bottom_left, bottom_right) {
                count += 1;
            }
        }
    }
    count
}
