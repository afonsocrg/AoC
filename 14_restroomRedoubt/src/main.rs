use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct RobotState {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
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

fn parse_input(input_file: &str) -> Vec<RobotState> {
    let line_pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
        .map(|line| {
            let captures = line_pattern.captures(line).unwrap();
            RobotState {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
                dx: captures[3].parse().unwrap(),
                dy: captures[4].parse().unwrap(),
            }
        })
        .collect()
}

fn part_1(input: Vec<RobotState>) -> i32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    // const WIDTH: i32 = 11;
    // const HEIGHT: i32 = 7;
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    input.into_iter().for_each(|robot| {
        let final_state = do_n_iterations(&robot, 100, WIDTH, HEIGHT);
        let (x, y) = (final_state.x, final_state.y);
        if y < HEIGHT / 2 {
            if x < WIDTH / 2 {
                q1 += 1;
            } else if x > WIDTH / 2 {
                q2 += 1;
            }
        } else if y > HEIGHT / 2 {
            if x < WIDTH / 2 {
                q3 += 1;
            } else if x > WIDTH / 2 {
                q4 += 1;
            }
        }
    });
    q1 * q2 * q3 * q4
}

fn part_2(input: Vec<RobotState>) -> i32 {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    let mut n_iter = 0;
    loop {
        let mut robot_positions = HashSet::new();
        input.iter().for_each(|robot| {
            let final_state = do_n_iterations(robot, n_iter, WIDTH, HEIGHT);
            robot_positions.insert((final_state.x, final_state.y));
        });
        if robot_positions.len() == input.len() {
            print_map(&robot_positions, WIDTH, HEIGHT);
            break;
        }
        n_iter += 1;
    }
    n_iter
}

fn do_n_iterations(state: &RobotState, n: i32, width: i32, height: i32) -> RobotState {
    RobotState {
        x: ((state.x + (state.dx * n)) % width + width) % width,
        y: ((state.y + (state.dy * n)) % height + height) % height,
        dx: state.dx,
        dy: state.dy,
    }
}

fn print_map(positions: &HashSet<(i32, i32)>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", if positions.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
}
