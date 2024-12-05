use regex::Regex;
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

fn parse_input(input_file: &str) -> String {
    fs::read_to_string(input_file).expect("Should have been able to read the file")
}

fn part_1(input: String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
        .sum()
}

fn part_2(input: String) -> i32 {
    let separator_regex = Regex::new(r"(do\(\)|don't\(\))").unwrap();

    let mut clean_input = String::new();
    let mut adding = true;
    let mut i = 0;

    for separator in separator_regex.find_iter(&input) {
        // println!("Separator: {}", separator.as_str());
        // println!("Adding: {}", adding);
        // println!("i: {}", i);

        if adding {
            clean_input += &input[i..separator.start()];
        }
        i = separator.end();
        adding = separator.as_str() == "do()";
    }
    if adding {
        clean_input += &input[i..];
    }

    // println!("Got clean input: '{}'", clean_input);

    part_1(clean_input)
}
