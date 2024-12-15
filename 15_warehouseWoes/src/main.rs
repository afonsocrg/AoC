use std::env;
use std::fs;
use std::time::Instant;
// use std::fs::File;
// use std::io::Write;

const WALL: char = '#';
const ROBOT: char = '@';
const OBJECT: char = 'O';
const EMPTY: char = '.';
const OBJECT_LEFT: char = '[';
const OBJECT_RIGHT: char = ']';

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

fn parse_input(input_file: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let binding = fs::read_to_string(input_file).expect("Failed to read file");
    let input = binding.split("\n\n").collect::<Vec<_>>();

    let map = input[0].lines().map(|l| l.chars().collect()).collect();
    let moves = input[1].replace("\n", "").chars().collect();

    (map, moves)
}

fn part_1((map, moves): (Vec<Vec<char>>, Vec<char>)) -> i32 {
    solve(map, moves, OBJECT)
}

fn part_2((map, moves): (Vec<Vec<char>>, Vec<char>)) -> i32 {
    solve(get_wide_map(&map), moves, OBJECT_LEFT)
}

fn solve(mut map: Vec<Vec<char>>, moves: Vec<char>, tracked_char: char) -> i32 {
    let mut robot_position = get_robot_position(&map);

    // Simulate the moves
    moves.iter().for_each(|m| {
        robot_position = move_robot(&mut map, robot_position, *m);
    });

    // Calculate GPS coordinates of the tracked character
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &c)| {
                    if c == tracked_char {
                        (i * 100 + j) as i32
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .sum()
}

fn get_robot_position(map: &Vec<Vec<char>>) -> (i32, i32) {
    map.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &c)| {
                if c == ROBOT {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

// Convert the map to a wide map where everything has double the width
fn get_wide_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = map[0].len();
    let height = map.len();
    let mut new_map = vec![vec![EMPTY; width * 2]; height];
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &c)| {
            let new_i = i;
            let new_j = 2 * j;
            match c {
                WALL => {
                    new_map[new_i][new_j] = WALL;
                    new_map[new_i][new_j + 1] = WALL;
                }
                ROBOT => {
                    new_map[new_i][new_j] = ROBOT;
                    new_map[new_i][new_j + 1] = EMPTY;
                }
                OBJECT => {
                    new_map[new_i][new_j] = OBJECT_LEFT;
                    new_map[new_i][new_j + 1] = OBJECT_RIGHT;
                }
                EMPTY => {}
                _ => unreachable!(),
            }
        });
    });
    new_map
}

fn move_robot(map: &mut Vec<Vec<char>>, (i, j): (i32, i32), direction: char) -> (i32, i32) {
    let (di, dj) = match direction {
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        '^' => (-1, 0),
        _ => unreachable!(),
    };

    // We need to check all the moving boxes before making a move
    // to prevent to move some of them too soon.
    // This is not required for Part 1 (see a more straightforward solution below)
    // but here I'm not too concerned about performance.
    if can_move(map, (i, j), (di, dj)) {
        move_rec(map, (i, j), (di, dj))
    } else {
        (i, j)
    }
}

fn can_move(map: &Vec<Vec<char>>, (i, j): (i32, i32), (di, dj): (i32, i32)) -> bool {
    let new_i = i + di;
    let new_j = j + dj;

    let what = map[new_i as usize][new_j as usize];
    match what {
        WALL => false,
        EMPTY => true,
        OBJECT => can_move(map, (new_i, new_j), (di, dj)),
        OBJECT_LEFT | OBJECT_RIGHT => match (di, dj) {
            (0, _) => can_move(map, (new_i, new_j), (di, dj)),
            (_, 0) => {
                let other_i = new_i;
                let other_j = if what == OBJECT_LEFT {
                    new_j + 1
                } else {
                    new_j - 1
                };
                can_move(map, (new_i, new_j), (di, dj))
                    && can_move(map, (other_i, other_j), (di, dj))
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn move_rec(map: &mut Vec<Vec<char>>, (i, j): (i32, i32), (di, dj): (i32, i32)) -> (i32, i32) {
    let new_i = i + di;
    let new_j = j + dj;

    let what = map[new_i as usize][new_j as usize];

    // First move the object that's in the target position
    match what {
        EMPTY => {}
        OBJECT => {
            move_rec(map, (new_i, new_j), (di, dj));
        }
        OBJECT_LEFT | OBJECT_RIGHT => match (di, dj) {
            (0, _) => {
                move_rec(map, (new_i, new_j), (di, dj));
            }
            (_, 0) => {
                let other_i = new_i;
                let other_j = if what == OBJECT_LEFT {
                    new_j + 1
                } else {
                    new_j - 1
                };
                move_rec(map, (new_i, new_j), (di, dj));
                move_rec(map, (other_i, other_j), (di, dj));
            }
            _ => unreachable!(),
        },
        WALL => panic!("Cannot move to ({}, {}): There is a wall!", new_i, new_j),
        _ => unreachable!(),
    };

    // Then move the current object to the new position
    map[new_i as usize][new_j as usize] = map[i as usize][j as usize];
    map[i as usize][j as usize] = EMPTY;

    // Return the new position
    (new_i, new_j)
}

// This was my first implementation for Part 1.
// Since it solves a simpler problem, I managed to make it slightly
// more efficient by reducing the number of writes:
// Here we just write the object that went to the empty space
// the robot in the new position, and the empty space in the robot's old position
// Keeping this commented to make the code simpler, and reuse the code from Part 2
// fn move_robot_part_1(
//     map: &mut Vec<Vec<char>>,
//     width: usize,
//     height: usize,
//     (i, j): (i32, i32),
//     direction: char,
// ) -> (i32, i32) {
//     let width = width as i32;
//     let height = height as i32;

//     let (di, dj) = match direction {
//         '>' => (0, 1),
//         '<' => (0, -1),
//         'v' => (1, 0),
//         '^' => (-1, 0),
//         _ => unreachable!(),
//     };

//     let mut empty_i = i + di;
//     let mut empty_j = j + dj;
//     while empty_i >= 0
//         && empty_i < height
//         && empty_j >= 0
//         && empty_j < width
//         && map[empty_i as usize][empty_j as usize] == OBJECT
//     {
//         empty_i += di;
//         empty_j += dj;
//     }

//     if empty_i < 0
//         || empty_i >= height
//         || empty_j < 0
//         || empty_j >= width
//         || map[empty_i as usize][empty_j as usize] != EMPTY
//     {
//         return (i, j);
//     }

//     let new_i = i + di;
//     let new_j = j + dj;

//     map[i as usize][j as usize] = EMPTY;

//     // Write robot position after object to make robot appear
//     // in cases where it does not move any box
//     map[empty_i as usize][empty_j as usize] = OBJECT;
//     map[new_i as usize][new_j as usize] = ROBOT;

//     (new_i, new_j)
// }

// Below is the code used to save intermediate states to some files,
// I just record states where the robot changed position, and remove
// portions of the walk where the robot goes to an already visited position
// without moving any box.

// #[derive(Clone)]
// struct State {
//     map: Vec<Vec<char>>,
//     robot_position: (i32, i32),
// }

// fn solve_trace_states(mut map: Vec<Vec<char>>, moves: Vec<char>, tracked_char: char) -> i32 {
//     let mut robot_position = get_robot_position(&map);

//     // Simulate the moves
//     let mut iteration = 0;
//     print_map_to_file(&map, format!("gifs/part2/map_{:06}.txt", iteration));
//     let mut buffered_states: Vec<State> = vec![];
//     moves.iter().for_each(|m| {
//         let prev_map = map.clone();
//         let new_position = move_robot(&mut map, robot_position, *m);
//         if new_position != robot_position {
//             if prev_map[new_position.0 as usize][new_position.1 as usize] == OBJECT_LEFT
//                 || prev_map[new_position.0 as usize][new_position.1 as usize] == OBJECT_RIGHT
//                 || prev_map[new_position.0 as usize][new_position.1 as usize] == OBJECT
//             {
//                 buffered_states.push(State {
//                     map: map.clone(),
//                     robot_position: new_position,
//                 });
//                 for s in buffered_states.iter() {
//                     iteration += 1;
//                     print_map_to_file(&s.map, format!("gifs/part2/map_{:06}.txt", iteration));
//                 }
//                 buffered_states.clear();
//             } else {
//                 // Check if there is a previous state with the current position
//                 // and clear the buffer from that state onwards
//                 let mut i = 0;
//                 while i < buffered_states.len() {
//                     if buffered_states[i].robot_position == new_position {
//                         buffered_states = buffered_states[..i].to_vec();
//                         break;
//                     }
//                     i += 1;
//                 }
//                 buffered_states.push(State {
//                     map: map.clone(),
//                     robot_position: new_position,
//                 });
//             }
//         }
//         robot_position = new_position;
//     });
// }

// fn print_map_to_file(map: &Vec<Vec<char>>, filename: String) {
//     let mut file = File::create(filename).expect("Failed to create file");
//     map.iter().for_each(|row| {
//         file.write_all(row.iter().collect::<String>().as_bytes())
//             .expect("Failed to write data");
//         file.write_all(b"\n").expect("Failed to write data");
//     });
// }