use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

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

fn parse_input(input_file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn part_1(input: Vec<Vec<char>>) -> u32 {
    let mut visited = HashSet::new();
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| {
                    // I could use the get_area_and_register_edges function, but I don't really
                    // need to register all the edges
                    let (area, perimeter) =
                        get_area_and_perimeter(&input, i as isize, j as isize, &mut visited);
                    area * perimeter
                })
                .sum::<u32>()
        })
        .sum()
}

fn get_area_and_perimeter(
    input: &Vec<Vec<char>>,
    i: isize,
    j: isize,
    visited: &mut HashSet<(isize, isize)>,
) -> (u32, u32) {
    if visited.contains(&(i, j)) {
        return (0, 0);
    }
    visited.insert((i, j));

    let mut area = 1;
    let mut perimeter = 0;

    for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let (new_i, new_j) = (i + di, j + dj);
        if new_i < 0
            || new_j < 0
            || new_i >= input.len() as isize
            || new_j >= input[0].len() as isize
            || input[new_i as usize][new_j as usize] != input[i as usize][j as usize]
        {
            perimeter += 1;
        } else {
            let (new_area, new_perimeter) = get_area_and_perimeter(input, new_i, new_j, visited);
            area += new_area;
            perimeter += new_perimeter;
        }
    }
    (area, perimeter)
}

fn part_2(input: Vec<Vec<char>>) -> u32 {
    let mut visited = HashSet::new();
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| {
                    let mut edges = HashSet::new();
                    let area = get_area_and_register_edges(
                        &input,
                        i as isize,
                        j as isize,
                        &mut visited,
                        &mut edges,
                    );
                    let n_sides = count_sides(&mut edges);
                    area * n_sides as u32
                })
                .sum::<u32>()
        })
        .sum()
}

fn get_area_and_register_edges(
    input: &Vec<Vec<char>>,
    i: isize,
    j: isize,
    visited: &mut HashSet<(isize, isize)>,
    edges: &mut HashSet<(isize, isize, isize, isize)>,
) -> u32 {
    if visited.contains(&(i, j)) {
        return 0;
    }
    visited.insert((i, j));

    let mut area = 1;

    for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let (new_i, new_j) = (i + di, j + dj);
        if new_i < 0
            || new_j < 0
            || new_i >= input.len() as isize
            || new_j >= input[0].len() as isize
            || input[new_i as usize][new_j as usize] != input[i as usize][j as usize]
        {
            edges.insert((i, j, di, dj));
        } else {
            area += get_area_and_register_edges(input, new_i, new_j, visited, edges);
        }
    }
    area
}

fn count_sides(edges: &mut HashSet<(isize, isize, isize, isize)>) -> u32 {
    let mut sides = edges.clone();

    for edge in edges.iter() {
        let (i, j, di, dj) = *edge;
        if !sides.contains(&(i, j, di, dj)) {
            continue;
        }
        // Remove all adjacent edges with the same direction
        for (ni, nj) in [(dj, di), (-dj, -di)] {
            let (mut new_i, mut new_j) = (i + ni, j + nj);
            while sides.remove(&(new_i, new_j, di, dj)) {
                new_i += ni;
                new_j += nj;
            }
        }
    }
    sides.len() as u32
}
