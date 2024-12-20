use std::collections::{HashMap, LinkedList};
use std::env;
use std::fmt::Debug;
use std::fs;
use std::time::Instant;

const OBSTACLE: char = '#';
const START: char = 'S';
const END: char = 'E';

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<char>>,
    width: i32,
    height: i32,
    start: (i32, i32),
    end: (i32, i32),
}

#[derive(Clone, Copy)]
struct State {
    position: (i32, i32),
    direction: (i32, i32),
    g: i32,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State(pos: {:?}, g: {})", self.position, self.g)
    }
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
    solve(&maze, 2)
}

fn part_2(maze: Maze) -> i32 {
    solve(&maze, 20)
}

fn solve(maze: &Maze, n: i32) -> i32 {
    // Cost to reach the end from each position
    let mut position_costs: HashMap<(i32, i32), i32> = HashMap::new();

    let mut path:LinkedList<State> = LinkedList::new();

    let mut s = State {
        position: maze.end,
        direction: (0, 1),
        g: 0,
    };

    // Calculate path + cost for each position in path
    // Starting from the end
    position_costs.insert(s.position, s.g);
    path.push_front(s);
    while s.position != maze.start {
        s = get_successor(&maze, &s);
        position_costs.insert(s.position, s.g);
        path.push_front(s);
    }

    count_shortcuts(&path, &position_costs, n, 100)
}

fn count_shortcuts(
    path: &LinkedList<State>,
    position_costs: &HashMap<(i32, i32), i32>,
    n: i32,
    threshold: i32,
) -> i32 {
    let mut count = 0;
    for state in path.iter() {

        // For each position within n steps away from the current position,
        // check if the cost difference is greater than the threshold
        for di in -n..=n {
            let k = n - di.abs();
            for dj in -k..=k {
                let next_pos = (state.position.0 + di, state.position.1 + dj);
                if let Some(cost) = position_costs.get(&next_pos) {
                    let saved_steps = (state.g - cost) - (di.abs() + dj.abs());
                    if saved_steps >= threshold {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

// Assuming that each position only has one successor
fn get_successor(maze: &Maze, state: &State) -> State {
    // First try to move forward
    let ns = next_state(state, state.direction);
    if is_valid_state(maze, &ns) {
        return ns;
    }

    // If it's not possible, try to go left and right
    let ns = next_state(state, turn_left(state.direction));
    if is_valid_state(maze, &ns) {
        return ns;
    }
    let ns = next_state(state, turn_right(state.direction));
    if is_valid_state(maze, &ns) {
        return ns;
    }

    unreachable!()
}

fn turn_left((di, dj): (i32, i32)) -> (i32, i32) {
    (-dj, di)
}

fn turn_right((di, dj): (i32, i32)) -> (i32, i32) {
    (dj, -di)
}

// Returns the state resulting from applying the move to the given state
fn next_state(state: &State, direction: (i32, i32)) -> State {
    State {
        position: (
            state.position.0 + direction.0,
            state.position.1 + direction.1,
        ),
        direction,
        g: state.g + 1,
    }
}

// Returns true if the state position is within the maze and not a wall
fn is_valid_state(maze: &Maze, state: &State) -> bool {
    let (i, j) = state.position;
    i >= 0
        && i < maze.height
        && j >= 0
        && j < maze.width
        && maze.map[i as usize][j as usize] != OBSTACLE
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
