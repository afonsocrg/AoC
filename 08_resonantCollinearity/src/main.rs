use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    i: i32,
    j: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Delta {
    di: i32,
    dj: i32,
}

impl std::ops::Add<Delta> for Position {
    type Output = Position;

    fn add(self, rhs: Delta) -> Self::Output {
        Position {
            i: self.i + rhs.di,
            j: self.j + rhs.dj,
        }
    }
}

impl std::ops::Sub<Delta> for Position {
    type Output = Position;

    fn sub(self, rhs: Delta) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Neg for Delta {
    type Output = Delta;

    fn neg(self) -> Self::Output {
        Delta {
            di: -self.di,
            dj: -self.dj,
        }
    }
}

impl std::ops::Div<i32> for Delta {
    type Output = Delta;

    fn div(self, rhs: i32) -> Self::Output {
        Delta {
            di: self.di / rhs,
            dj: self.dj / rhs,
        }
    }
}

impl std::ops::Mul<i32> for Delta {
    type Output = Delta;

    fn mul(self, rhs: i32) -> Self::Output {
        Delta {
            di: self.di * rhs,
            dj: self.dj * rhs,
        }
    }
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
        .expect("Failed to read file")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn part_1(input: Vec<Vec<char>>) -> i32 {
    solve(input, false)
}

fn part_2(input: Vec<Vec<char>>) -> i32 {
    solve(input, true)
}

fn solve(mut input: Vec<Vec<char>>, part_2: bool) -> i32 {
    let antennas = get_antennas(&input);
    let n_rows = input.len();
    let n_cols = input[0].len();

    let result = find_antinodes(&antennas, n_rows as i32, n_cols as i32, part_2)
        .into_iter()
        .filter(|a| a.i >= 0 && a.i < n_rows as i32 && a.j >= 0 && a.j < n_cols as i32)
        .map(|a| {
            println!("({},{})", a.i, a.j);
            if input[a.i as usize][a.j as usize] == '.' {
                input[a.i as usize][a.j as usize] = '#';
            }
            a
        })
        .count() as i32;

    print_map(&input);
    result
}

fn get_antennas(map: &Vec<Vec<char>>) -> HashMap<char, HashSet<Position>> {
    let mut antennas = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &freq) in row.iter().enumerate() {
            if regex::Regex::new(r"[a-zA-Z0-9]")
                .unwrap()
                .is_match(freq.to_string().as_str())
            {
                antennas
                    .entry(freq)
                    .or_insert(HashSet::new())
                    .insert(Position {
                        i: i as i32,
                        j: j as i32,
                    });
            }
        }
    }
    antennas
}

fn find_antinodes(
    antennas: &HashMap<char, HashSet<Position>>,
    max_rows: i32,
    max_cols: i32,
    part_2: bool,
) -> HashSet<Position> {
    let mut antinodes = HashSet::new();

    for positions in antennas.values() {
        for p1 in positions {
            for p2 in positions {
                if p1 == p2 {
                    continue;
                }

                let di = p2.i as i32 - p1.i as i32;
                let dj = p2.j as i32 - p1.j as i32;

                let delta = Delta { di, dj };

                if !part_2 {
                    antinodes.insert(*p2 + delta);
                    antinodes.insert(*p1 - delta);
                } else {
                    // The antenas themselves are antinodes
                    antinodes.insert(*p1);
                    antinodes.insert(*p2);

                    // Add all antinodes in line with the two antennas
                    // that are within the grid
                    let mut pi = *p2 + delta;
                    while in_bounds(pi, max_rows, max_cols) {
                        antinodes.insert(pi);
                        pi = pi + delta;
                    }

                    pi = *p1 - delta;
                    while in_bounds(pi, max_rows, max_cols) {
                        antinodes.insert(pi);
                        pi = pi - delta;
                    }
                }

                // Should not forget antinodes *between* antennas
                if di % 3 == 0 && dj % 3 == 0 {
                    let delta_third = delta / 3;
                    antinodes.insert(*p1 + delta_third);
                    antinodes.insert(*p2 - delta_third);
                }
            }
        }
    }
    antinodes
}

fn in_bounds(p: Position, max_rows: i32, max_cols: i32) -> bool {
    p.i >= 0 && p.i < max_rows && p.j >= 0 && p.j < max_cols
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}
