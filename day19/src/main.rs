use std::error::Error;

use aoc::input::parse_input;
use fxhash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let (available, designs) = parse_input(19, parse_input_str)?;

    // Get number of valid arrangements for each design
    let composable = build_composable(&available, &designs);

    // Run parts
    println!("Part 1: {}", part1(&composable));
    println!("Part 2: {}", part2(&composable));

    Ok(())
}

fn part1(composable: &[usize]) -> u64 {
    // Count the number of designs with at least one valid arrangement
    composable.iter().filter(|c| **c != 0).count() as u64
}

fn part2(composable: &[usize]) -> u64 {
    // Sum the total number of arrangements
    composable.iter().sum::<usize>() as u64
}

fn build_composable(available: &FxHashSet<String>, designs: &[String]) -> Vec<usize> {
    // Get max length of available
    let maxlen = available.iter().map(|a| a.len()).max().unwrap();

    designs
        .iter()
        .map(|design| {
            // Get the design length
            let dlen = design.len();

            // Create vector to hold number of composable for each position
            let mut composable = vec![0usize; dlen];

            // Loop each design position in reverse
            for idx in (0..dlen).rev() {
                // Calculate max number of characters to scan
                let max_scan = maxlen.min(dlen - idx);

                // Scan all substrings
                for sublen in 1..=max_scan {
                    // Get substring
                    let substr = &design[idx..idx + sublen];

                    // Substring in available?
                    if let Some(available) = available.get(substr) {
                        // Yes - calculate next position
                        let next_idx = idx + available.len();

                        if next_idx == dlen {
                            // Reached the end
                            composable[idx] += 1;
                        } else {
                            // Add the number of composable at the next index
                            composable[idx] += composable[next_idx];
                        }
                    }
                }
            }

            // Return the number of composable at position 0
            composable[0]
        })
        .collect()
}

// Input parsing

fn parse_input_str(input: &str) -> (FxHashSet<String>, Vec<String>) {
    let mut sections = input.split("\n\n");

    let available = sections.next().unwrap();

    let available = available
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    let designs = sections.next().unwrap();

    let designs = designs.lines().map(|s| s.to_string()).collect();

    (available, designs)
}

#[cfg(test)]
mod tests;
