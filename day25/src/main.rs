use std::error::Error;

use aoc::input::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(25).unwrap();
    let (locks, keys) = parse_input(&input);

    // Run part
    println!("Part 1: {}", part1(&locks, &keys));

    Ok(())
}

fn part1(locks: &[Lock], keys: &[Key]) -> u64 {
    // Iterate each lock
    locks
        .iter()
        .map(|lock| {
            // Iterate each key
            keys.iter()
                .filter(|key| {
                    // Does this key fit in this lock?
                    lock.heights
                        .iter()
                        .zip(key.heights.iter())
                        .map(|(l, k)| l + k)
                        .all(|s| s <= 5)
                })
                .count() as u64
        })
        .sum()
}

// Input parsing

#[derive(Debug)]
struct Key {
    heights: Vec<u8>,
}

#[derive(Debug)]
struct Lock {
    heights: Vec<u8>,
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let blocks = input.split("\n\n");

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for block in blocks {
        let line1 = block.lines().next().unwrap();
        let len = line1.len();
        let lock = line1.starts_with('#');

        let iter: Box<dyn Iterator<Item = &str>> = if lock {
            Box::new(block.lines())
        } else {
            Box::new(block.lines().rev())
        };

        let heights = iter
            .enumerate()
            .fold(vec![0; len], |mut heights, (y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    if c == '#' {
                        heights[x] = y as u8;
                    }
                });

                heights
            });

        if lock {
            locks.push(Lock { heights })
        } else {
            keys.push(Key { heights })
        }
    }

    (locks, keys)
}

#[cfg(test)]
mod tests;
