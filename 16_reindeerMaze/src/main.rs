use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fmt::Debug;
use std::fs;
use std::time::Instant;

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

#[derive(Debug)]
enum Move {
    Forward,
    TurnLeft,
    TurnRight,
}

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
    direction: (i32, i32),
    g: i32,
    h: i32,
    predecessor: Option<Box<State>>,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_char = match self.direction {
            (0, 1) => '>',
            (0, -1) => '<',
            (-1, 0) => '^',
            (1, 0) => 'v',
            _ => unreachable!(),
        };
        write!(
            f,
            "State(pos: {:?}, dir: {:?}, g: {}, h: {})",
            self.position, dir_char, self.g, self.h
        )
    }
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
        self.g == other.g
            && self.h == other.h
            && self.position == other.position
            && self.direction == other.direction
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
    println!("{:?}", result);
    println!("Executed in {:?}", elapsed);
}

fn parse_input(input_file: &str) -> Maze {
    let map = fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let start = find_first(&map, START).unwrap();
    let end = find_first(&map, END).unwrap();

    let width = map[0].len() as i32;
    let height = map.len() as i32;

    Maze {
        map,
        start,
        end,
        width,
        height,
    }
}

fn part_1(maze: Maze) -> i32 {
    let final_states = a_star(&maze, false);
    let final_state = final_states.get(0).expect("No final state found");
    final_state.g
}

fn part_2(maze: Maze) -> i32 {
    let mut best_tiles: HashSet<(i32, i32)> = HashSet::new();
    for final_state in a_star(&maze, true) {
        let mut s = Some(final_state.clone());
        while let Some(st) = s {
            let (i, j) = st.position;
            best_tiles.insert((i, j));
            s = st.predecessor;
        }
    }
    best_tiles.len() as i32
}

fn a_star(maze: &Maze, find_all: bool) -> Vec<Box<State>> {
    let mut states: BinaryHeap<Box<State>> = BinaryHeap::new();

    // Since we already know our goal, we can pre-calculate the heuristic for every position
    let heuristic_cache = calculate_heuristics(maze, maze.end);

    let mut initial_state = Box::new(State {
        position: maze.start,
        direction: (0, 1),
        g: 0,
        h: 0,
        predecessor: None,
    });
    initial_state.h = *heuristic_cache
        .get(&initial_state.position)
        .expect("No heuristic found for initial state");

    let mut final_states: Vec<Box<State>> = Vec::new();
    let mut best_f: Option<i32> = None;

    states.push(initial_state.clone());

    while let Some(state) = states.pop() {
        // If we already found a solution and the current state has a higher heuristic,
        // We can stop the search (because the heuristic is admissible)
        if best_f.is_some_and(|best_f| best_f < state.g + state.h) {
            break;
        }

        if state.position == maze.end {
            final_states.push(state.clone());
            if best_f.is_none() {
                best_f = Some(state.g + state.h);
            }
            if !find_all {
                return final_states;
            }
        }

        for mut next_state in expand_state(maze, &state) {
            // We know that the next_state will only reach the goal if it has a heuristic value
            if let Some(h) = heuristic_cache.get(&next_state.position) {
                next_state.h = *h;
                states.push(next_state.clone());
            }
        }
    }

    final_states
}

// Returns all *useful* neighbors of the current state, i.e. the next states that
// have at least one bifurcation
fn expand_state_rec(maze: &Maze, state: &Box<State>, lookahead: bool) -> Vec<Box<State>> {
    let mut next_states = Vec::new();
    for m in [Move::Forward, Move::TurnLeft, Move::TurnRight] {
        let mut next_state = next_state(state, m);

        if !is_valid_state(maze, &next_state) {
            continue;
        }

        if lookahead {
            // Expand next state while it only has one option
            // If we find a dead end, do not include that next state
            // If we find a bifurcation, include that state
            let mut lookahead_states = expand_state_rec(maze, &next_state, false);
            while next_state.position != maze.end && lookahead_states.len() == 1 {
                next_state = lookahead_states.pop().unwrap();
                lookahead_states = expand_state_rec(maze, &next_state, false);
            }
            if lookahead_states.len() == 0 {
                // We found a dead end
                continue;
            }
        }
        next_states.push(next_state);
    }
    next_states
}

fn expand_state(maze: &Maze, state: &Box<State>) -> Vec<Box<State>> {
    expand_state_rec(maze, state, true)
}

// Returns the state resulting from applying the move to the given state
fn next_state(state: &Box<State>, m: Move) -> Box<State> {
    match m {
        Move::Forward => Box::new(State {
            position: (
                state.position.0 + state.direction.0,
                state.position.1 + state.direction.1,
            ),
            direction: state.direction,
            g: state.g + 1,
            h: 0,
            predecessor: Some(state.clone()),
        }),
        Move::TurnLeft => {
            let new_direction = (-state.direction.1, state.direction.0);
            let new_position = (
                state.position.0 + new_direction.0,
                state.position.1 + new_direction.1,
            );
            Box::new(State {
                position: new_position,
                direction: new_direction,
                g: state.g + 1001,
                h: 0,
                predecessor: Some(state.clone()),
            })
        }
        Move::TurnRight => {
            let new_direction = (state.direction.1, -state.direction.0);
            let new_position = (
                state.position.0 + new_direction.0,
                state.position.1 + new_direction.1,
            );
            Box::new(State {
                position: new_position,
                direction: new_direction,
                g: state.g + 1001,
                h: 0,
                predecessor: Some(state.clone()),
            })
        }
    }
}

// Returns true if the state position is within the maze and not a wall
fn is_valid_state(maze: &Maze, state: &State) -> bool {
    let (i, j) = state.position;
    i >= 0
        && i < maze.height
        && j >= 0
        && j < maze.width
        && maze.map[i as usize][j as usize] != WALL
}

// This function calculates an admissible heuristic for every position in the maze
// We do that by measuring the cost of going from the goal position to every other position
// Having this heuristic is crucial to speed up the A* search!!
fn calculate_heuristics(maze: &Maze, goal: (i32, i32)) -> HashMap<(i32, i32), i32> {
    let mut heuristics: HashMap<(i32, i32), i32> = HashMap::new();

    // Using a Binary Heap to always expand the state with the lowest cost first
    let mut queue: BinaryHeap<Box<State>> = BinaryHeap::new();

    // Make sure we go to all directions from the goal position
    for direction in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
        queue.push(Box::new(State {
            position: goal,
            direction,
            g: 0,
            h: 0,
            predecessor: None,
        }));
    }

    // Expand the states until we have visited all positions
    while let Some(state) = queue.pop() {
        let (i, j) = state.position;
        heuristics.insert((i, j), state.g);

        for m in [Move::Forward, Move::TurnLeft, Move::TurnRight] {
            let next_state = next_state(&state, m);
            if is_valid_state(maze, &next_state) && !heuristics.contains_key(&next_state.position) {
                queue.push(next_state);
            }
        }
    }

    heuristics
}

fn find_first(maze: &Vec<Vec<char>>, char: char) -> Option<(i32, i32)> {
    maze.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, &c)| {
            if c == char {
                Some((i as i32, j as i32))
            } else {
                None
            }
        })
    })
}
