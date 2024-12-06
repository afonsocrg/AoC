use std::collections::HashSet;
use std::env;
use std::fs;

const DEBUG: bool = false;

const OBSTACLE: char = '#';
const VISITED: char = 'X';
const TESTED: char = '?';
const FAKE_OBSTACLE: char = 'O';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    i: isize,
    j: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direction {
    di: isize,
    dj: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Position,
    direction: Direction,
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

fn parse_input(input_file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input_file)
        .expect("Could not read file")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn part_1(mut input: Vec<Vec<char>>) -> usize {
    let mut state = get_initial_state(&input).unwrap();
    let mut visited: HashSet<Position> = HashSet::new();

    while !is_outside(&input, &state.position) {
        visited.insert(state.position);
        if DEBUG {
            input[state.position.i as usize][state.position.j as usize] = VISITED;
            print_input(&input, &state.position, &state.direction, None);
        }
        state = transition_state(&input, &state);
    }
    print_input(&input, &state.position, &state.direction, None);
    visited.len()
}

fn part_2(mut input: Vec<Vec<char>>) -> usize {
    let mut state = get_initial_state(&input).unwrap();
    let initial_position = state.position;

    let mut visited_states: HashSet<State> = HashSet::new();
    let mut visited_positions: HashSet<Position> = HashSet::new();

    let mut bad_positions: HashSet<Position> = HashSet::new();

    while !is_outside(&input, &state.position) {
        visited_states.insert(state.clone());
        visited_positions.insert(state.position);

        if DEBUG {
            input[state.position.i as usize][state.position.j as usize] = VISITED;
            print_input(&input, &state.position, &state.direction, None);
        }

        // test if placing an obstacle in front of us would create a loop
        let obstacle_position = next_position(&state.position, &state.direction);
        let test_direction = turn_right(state.direction);

        if !is_outside(&input, &obstacle_position)
            && obstacle_position != initial_position
            && input[obstacle_position.i as usize][obstacle_position.j as usize] != OBSTACLE
            && !visited_positions.contains(&obstacle_position)
        {
            if DEBUG {
                println!(
                    "Testing hypothetical obstacle at ({}, {})",
                    obstacle_position.i, obstacle_position.j
                );
                input[obstacle_position.i as usize][obstacle_position.j as usize] = FAKE_OBSTACLE;
            }
            let y = obstacle_position.i as usize;
            let x = obstacle_position.j as usize;
            let prev_char = input[y][x];
            input[y][x] = OBSTACLE;
            if has_loop(
                &input,
                &visited_states,
                &State {
                    position: state.position,
                    direction: test_direction,
                },
            ) {
                bad_positions.insert(obstacle_position);
            }
            input[y][x] = prev_char;
        } else {
            if DEBUG {
                println!(
                    "Not testing obstacle at ({}, {})",
                    obstacle_position.j, obstacle_position.i
                );
            }
        }

        state = transition_state(&input, &state);
    }

    // I used this print to compare with the output of the bruteforce algorithm
    // println!("{:?}", &bad_positions);
    bad_positions.len()
}

fn has_loop(input: &Vec<Vec<char>>, visited_states: &HashSet<State>, state: &State) -> bool {
    let mut input = input.clone();
    let mut visited_states = visited_states.clone();
    let mut state = state.clone();

    while !is_outside(&input, &state.position) {
        if DEBUG {
            if input[state.position.i as usize][state.position.j as usize] != FAKE_OBSTACLE {
                input[state.position.i as usize][state.position.j as usize] = TESTED;
            }
        }
        if visited_states.contains(&state) {
            if DEBUG {
                print_input(&input, &state.position, &state.direction, Some(4));
                println!(" <======================= LOOP DETECTED");
            }
            return true;
        }

        visited_states.insert(state.clone());
        state = transition_state(&input, &state);
    }
    if DEBUG {
        print_input(&input, &state.position, &state.direction, Some(4));
        println!("No loop");
    }
    false
}

fn transition_state(map: &Vec<Vec<char>>, s: &State) -> State {
    if is_facing_obstacle(&map, &s) {
        // If we are facing an obstacle, we turn right, but stay in the same position
        // This is important because on our right we may have another obstacle!!
        State {
            position: s.position,
            direction: turn_right(s.direction),
        }
    } else {
        State {
            position: next_position(&s.position, &s.direction),
            direction: s.direction,
        }
    }
}

fn next_position(p: &Position, d: &Direction) -> Position {
    Position {
        j: p.j + d.dj,
        i: p.i + d.di,
    }
}

fn get_initial_state(input: &Vec<Vec<char>>) -> Option<State> {
    for (i, row) in input.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '^' || c == 'v' || c == '<' || c == '>' {
                return Some(State {
                    position: Position {
                        j: j as isize,
                        i: i as isize,
                    },
                    direction: get_char_dir(c),
                });
            }
        }
    }
    None
}

fn is_facing_obstacle(input: &Vec<Vec<char>>, s: &State) -> bool {
    let new_p = next_position(&s.position, &s.direction);
    if is_outside(input, &new_p) {
        return false;
    }
    input[new_p.i as usize][new_p.j as usize] == OBSTACLE
}

fn is_outside(input: &Vec<Vec<char>>, p: &Position) -> bool {
    let max_i = input.len() as isize;
    let max_j = input[0].len() as isize;
    p.i < 0 || p.i >= max_i || p.j < 0 || p.j >= max_j
}

fn turn_right(d: Direction) -> Direction {
    Direction {
        dj: -d.di,
        di: d.dj,
    }
}

fn get_dir_char(d: &Direction) -> char {
    match d {
        Direction { di: -1, dj: 0 } => '^',
        Direction { di: 1, dj: 0 } => 'v',
        Direction { di: 0, dj: -1 } => '<',
        Direction { di: 0, dj: 1 } => '>',
        _ => panic!("Invalid direction"),
    }
}

fn get_char_dir(c: char) -> Direction {
    match c {
        '^' => Direction { di: -1, dj: 0 },
        'v' => Direction { di: 1, dj: 0 },
        '<' => Direction { di: 0, dj: -1 },
        '>' => Direction { di: 0, dj: 1 },
        _ => panic!("Invalid direction"),
    }
}

fn print_input(input: &Vec<Vec<char>>, p: &Position, d: &Direction, indent: Option<usize>) {
    if !DEBUG {
        return;
    }

    let indent = indent.unwrap_or(0);
    let dir_char = get_dir_char(d);
    println!("({},{}): {}, ({},{})", p.i, p.j, dir_char, d.di, d.dj);

    for (i, row) in input.iter().enumerate() {
        for _ in 0..indent {
            print!(" ");
        }
        print!("{:03}: ", i);
        for (j, &c) in row.iter().enumerate() {
            if (Position {
                j: j as isize,
                i: i as isize,
            }) == *p
            {
                print!("{}", dir_char);
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}

// This bruteforce algorithm allowed me to understand why the other algorithm was failing
// By comparing its output with this one I was able to identify the bug
fn _part_2_brute_force(mut input: Vec<Vec<char>>) -> usize {
    let state = get_initial_state(&input).unwrap();
    let initial_position = state.position;

    let mut bad_positions: HashSet<Position> = HashSet::new();

    let n_rows = input.len();
    let n_cols = input[0].len();

    for i in 0..n_rows {
        for j in 0..n_cols {
            let p = Position {
                j: j as isize,
                i: i as isize,
            };
            if input[i][j] == OBSTACLE || p == initial_position {
                continue;
            }
            let prev_char = input[i][j];
            input[i][j] = OBSTACLE;
            if has_loop(&input, &HashSet::new(), &state) {
                bad_positions.insert(p);
            }
            input[i][j] = prev_char;
        }
    }

    // I used this print to compare with the output of the other algorithm
    // println!("{:?}", &bad_positions);
    bad_positions.len()
}
