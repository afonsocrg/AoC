use std::collections::{BinaryHeap, HashSet};
use std::env;
use std::fs;
use std::time::Instant;

const OBSTACLE: char = '#';
const EMPTY: char = ' ';

const SIZE: i32 = 71;
// const SIZE: i32 = 7;

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<char>>,
    width: i32,
    height: i32,
    start: (i32, i32),
    end: (i32, i32),
}

#[derive(Clone)]
struct State {
    position: (i32, i32),
    g: i32,
    h: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.g + self.h).cmp(&(other.g + other.h)).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.g + self.h).cmp(&(other.g + other.h)).reverse())
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.g == other.g && self.h == other.h && self.position == other.position
    }
}

impl Eq for State {}

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
    println!("{}", result);
    println!("Executed in {:?}", elapsed);
}

fn parse_input(input_file: &str) -> Vec<(i32, i32)> {
    fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect()
}

fn part_1(obstacles: Vec<(i32, i32)>) -> String {
    a_star(&Maze {
        map: generate_map(&obstacles, 1024),
        start: (0, 0),
        end: (SIZE - 1, SIZE - 1),
        width: SIZE,
        height: SIZE,
    })
    .expect("No final state found")
    .g
    .to_string()
}

fn part_2(obstacles: Vec<(i32, i32)>) -> String {
    let mut i = 0;
    let mut j = obstacles.len() as i32;
    while i < j {
        let mid = (j + i) / 2;
        if let Some(_) = a_star(&Maze {
            map: generate_map(&obstacles, mid),
            start: (0, 0),
            end: (SIZE - 1, SIZE - 1),
            width: SIZE,
            height: SIZE,
        }) {
            i = mid + 1;
        } else {
            j = mid;
        }
    }
    format!("{},{}", obstacles[i as usize].0, obstacles[i as usize].1)
}

fn a_star(maze: &Maze) -> Option<State> {
    let mut states: BinaryHeap<State> = BinaryHeap::new();

    let mut visited = HashSet::new();

    let mut initial_state = State {
        position: maze.start,
        g: 0,
        h: 0,
    };
    initial_state.h = manhattan_distance(maze.start, maze.end);

    states.push(initial_state);

    while let Some(state) = states.pop() {
        if state.position == maze.end {
            return Some(state);
        }

        for mut next_state in expand_state(maze, &state) {
            // We know that the next_state will only reach the goal if it has a heuristic value
            next_state.h = manhattan_distance(next_state.position, maze.end);
            if visited.contains(&next_state.position) {
                continue;
            }
            states.push(next_state);
        }
        visited.insert(state.position);
    }

    None
}

fn expand_state(maze: &Maze, state: &State) -> Vec<State> {
    let mut next_states = Vec::new();
    for m in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
        let next_state = next_state(state, m);
        if is_valid_state(maze, &next_state) {
            next_states.push(next_state);
        }
    }
    next_states
}

// Returns the state resulting from applying the move to the given state
fn next_state(state: &State, direction: (i32, i32)) -> State {
    State {
        position: (
            state.position.0 + direction.0,
            state.position.1 + direction.1,
        ),
        g: state.g + 1,
        h: 0,
    }
}

// Returns true if the state position is within the maze and not a wall
fn is_valid_state(maze: &Maze, state: &State) -> bool {
    let (j, i) = state.position;
    i >= 0
        && i < maze.height
        && j >= 0
        && j < maze.width
        && maze.map[i as usize][j as usize] != OBSTACLE
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn generate_map(obstacles: &Vec<(i32, i32)>, timestep: i32) -> Vec<Vec<char>> {
    let mut map = vec![vec![EMPTY; SIZE as usize]; SIZE as usize];

    for i in 0..=timestep {
        map[obstacles[i as usize].1 as usize][obstacles[i as usize].0 as usize] = OBSTACLE;
    }

    map
}
