#![feature(linked_list_cursors)]

use std::cmp;
use std::collections::LinkedList;
use std::env;
use std::fs;

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

fn parse_input(input_file: &str) -> Vec<char> {
    fs::read_to_string(input_file)
        .expect("Failed to read input file")
        .chars()
        .collect()
}

fn part_1(mut input: Vec<char>) -> u64 {
    let mut checksum: u64 = 0;
    let mut i = 0; // iterator that's moving forward
    let mut j = input.len() - 1; // iterator that's moving backward
    let mut curr_offset: u64 = 0;
    while i <= j {
        let i_is_file = i % 2 == 0;
        if i_is_file {
            let i_file_id = i / 2;
            let i_file_len = input[i].to_digit(10).expect("Failed to parse character");
            checksum += segment_checksum(curr_offset, i_file_len as u64, i_file_id as u64);

            i += 1;
            curr_offset += i_file_len as u64;
        } else {
            let mut free_blocks = input[i]
                .to_digit(10)
                .expect("Failed to parse character")
                .into();
            let j_is_file = j % 2 == 0;
            while free_blocks > 0 && j > i {
                if !j_is_file {
                    j -= 1;
                    // We need to check again if j > i
                    continue;
                }

                // j is a file
                let j_file_id = j / 2;
                let j_file_len = input[j].to_digit(10).expect("Failed to parse character");
                let n_blocks: u32 = cmp::min(free_blocks, j_file_len);

                checksum += segment_checksum(curr_offset, n_blocks as u64, j_file_id as u64);

                curr_offset += n_blocks as u64;

                let remaining_j_blocks = j_file_len - n_blocks;
                input[j] =
                    char::from_digit(remaining_j_blocks, 10).expect("Failed to convert to char");

                free_blocks -= n_blocks;
                if remaining_j_blocks == 0 {
                    j -= 2;
                }
            }
            i += 1;
        }
    }
    checksum
}

fn part_2(input: Vec<char>) -> u64 {
    let mut memory_map = get_memory_map(input);
    let max_space_size = memory_map.max_space_size;

    // Reallocate files
    for file in &mut memory_map.files {
        if file.len > max_space_size {
            continue;
        }

        // TODO: Maybe update max_space_size after reallocating file
        // It was not necessary for my input size
        let mut space_cursor = memory_map.spaces.cursor_front_mut();
        while let Some(space) = space_cursor.current() {
            if space.offset > file.offset {
                break;
            }
            if space.len >= file.len {
                // Found space for file
                file.offset = space.offset;
                if file.len == space.len {
                    space_cursor.remove_current();
                } else {
                    space.len -= file.len;
                    space.offset += file.len as u64;
                }
                break;
            }
            space_cursor.move_next();
        }
    }

    // Calculate checksum
    memory_map
        .files
        .iter()
        .map(|file| segment_checksum(file.offset, file.len.try_into().unwrap(), file.file_id))
        .sum()
}

// Calculates the checksum of a memory segment (with only one file)
fn segment_checksum(offset: u64, chunk_size: u64, file_id: u64) -> u64 {
    ((chunk_size * chunk_size) + (2 * offset * chunk_size) - chunk_size) * file_id / 2
}

#[derive(Debug)]
struct File {
    file_id: u64,
    len: usize,
    offset: u64,
}

#[derive(Debug)]
struct Space {
    offset: u64,
    len: usize,
}

#[derive(Debug)]
struct MemoryMap {
    files: LinkedList<File>,
    spaces: LinkedList<Space>,
    max_space_size: usize,
}

fn get_memory_map(input: Vec<char>) -> MemoryMap {
    let mut memory_map: MemoryMap = MemoryMap {
        files: LinkedList::new(),
        spaces: LinkedList::new(),
        max_space_size: 0,
    };
    let mut curr_offset: u64 = 0;
    for i in 0..input.len() {
        let chunk_size: usize = input[i]
            .to_digit(10)
            .expect("Failed to parse character")
            .try_into()
            .unwrap();
        if i % 2 == 0 {
            memory_map.files.push_front(File {
                file_id: (i / 2) as u64,
                len: chunk_size,
                offset: curr_offset,
            })
        } else {
            if chunk_size > 0 {
                memory_map.spaces.push_back(Space {
                    offset: curr_offset,
                    len: chunk_size,
                });

                memory_map.max_space_size =
                    cmp::max(memory_map.max_space_size, chunk_size as usize);
            }
        }
        curr_offset += chunk_size as u64;
    }
    memory_map
}
