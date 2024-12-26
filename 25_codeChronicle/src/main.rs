use std::collections::HashMap;
use std::fmt::Debug;
use std::env;
use std::fs;
use std::time::Instant;

const MAX_PIN_HEIGHT: u8 = 5;
const MAX_PIN_WIDTH: usize = 5;

type Key = [u8; MAX_PIN_WIDTH];
type Lock = [u8; MAX_PIN_WIDTH];


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
    println!("Answer: {}", result);
    println!("Executed in {:?}", elapsed);
}

fn parse_input(input_file: &str) -> (Vec<Key>, Vec<Lock>) {
    let binding = fs::read_to_string(input_file).expect("Failed to read file");
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    binding.split("\n\n").for_each(|s| {
        let lines = s.lines().collect::<Vec<&str>>();
        let n_lines = lines.len();
        let n_pins = lines[0].len();
        assert_eq!(n_pins, 5);

        let is_key = lines[0].contains('.');
        let grid = lines
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut sizes = [0; MAX_PIN_WIDTH];
        for j in 0..n_pins {
            let mut pin_size: u8 = 0;
            for i in 1..n_lines - 1 {
                if grid[i][j] == '#' {
                    pin_size += 1;
                }
            }
            sizes[j] = pin_size;
        }
        if is_key {
            keys.push(sizes);
        } else {
            locks.push(sizes);
        }
    });

    (keys, locks)
}


struct LockTree {
    children: HashMap<u8, Box<LockTree>>,
}

impl Debug for LockTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.children)
    }
}

fn part_1((keys, locks): (Vec<Key>, Vec<Lock>)) -> String {
    let mut lock_tree = Box::new(LockTree {
        children: HashMap::new(),
    });

    // Insert locks into the tree
    for lock in locks {
        let mut current_node = &mut lock_tree;
        for pin in lock {
            let child = current_node.children.entry(pin).or_insert_with(|| {
                Box::new(LockTree {
                    children: HashMap::new(),
                })
            });
            current_node = child;
        }
    }

    // Count number of possible key/lock combinations
    let mut count = 0;
    for key in keys {
        count += count_combinations(&lock_tree, &key);
    }
    count.to_string()
}

fn count_combinations(lock_tree: &LockTree, key: &[u8]) -> u32 {
    if key.is_empty() {
        return 1;
    }

    let mut count = 0;
    let pin = key[0];
    let max_lock_height = MAX_PIN_HEIGHT - pin;
    for i in 0..=max_lock_height {
        if let Some(child) = lock_tree.children.get(&i) {
            count += count_combinations(child, &key[1..]);
        }
    }
    count
}

fn part_2(_: (Vec<Key>, Vec<Lock>)) -> String {
    "Done! :)".to_string()
}
