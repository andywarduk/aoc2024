use std::{collections::VecDeque, error::Error};

use aoc::input::parse_input_line;
use fxhash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(11, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &InputEnt) -> u64 {
    count(input, 25)
}

fn part2(input: &InputEnt) -> u64 {
    count(input, 75)
}

fn count(input: &InputEnt, iters: u8) -> u64 {
    let mut work = VecDeque::new();
    let mut note = FxHashMap::default();

    // Build initial work queue
    (0..input.len()).for_each(|i| {
        work.push_back((input[i], iters));
    });

    // Process work queue
    while let Some((num, iters)) = work.pop_front() {
        let mut rework = true;

        if num == 0 {
            // 0 -> 1
            if iters == 1 {
                note.insert((num, iters), 1u64);
                rework = false;
            } else if let Some(count) = note.get(&(1, iters - 1)) {
                note.insert((num, iters), *count);
                rework = false;
            } else {
                work.push_front((1, iters - 1));
            }
        } else {
            let log10 = num.ilog10();

            if (log10 & 1) == 1 {
                // Split even number
                if iters == 1 {
                    note.insert((num, iters), 2u64);
                    rework = false;
                } else {
                    let div = 10u64.pow((log10 + 1) / 2);

                    let num1 = num / div;
                    let num2 = num % div;

                    match (note.get(&(num1, iters - 1)), note.get(&(num2, iters - 1))) {
                        (Some(count1), Some(count2)) => {
                            note.insert((num, iters), count1 + count2);
                            rework = false;
                        }
                        (Some(_), None) => {
                            work.push_front((num2, iters - 1));
                        }
                        (None, Some(_)) => {
                            work.push_front((num1, iters - 1));
                        }
                        (None, None) => {
                            work.push_front((num2, iters - 1));
                            work.push_front((num1, iters - 1));
                        }
                    }
                }
            } else {
                // Odd number - multiply by 2024
                if iters == 1 {
                    note.insert((num, iters), 1u64);
                    rework = false;
                } else {
                    let new = num * 2024;

                    if let Some(count) = note.get(&(new, iters - 1)) {
                        note.insert((num, iters), *count);
                        rework = false;
                    } else {
                        work.push_front((new, iters - 1));
                    }
                }
            }
        }

        if rework {
            work.push_back((num, iters));
        }
    }

    input
        .iter()
        .map(|num| *note.get(&(*num, iters)).unwrap())
        .sum()
}

// Input parsing

type InputEnt = Vec<u64>;

fn input_transform(line: &str) -> InputEnt {
    line.split_ascii_whitespace()
        .map(|ns| ns.parse::<u64>().expect("not an integer"))
        .collect()
}

#[cfg(test)]
mod tests;
