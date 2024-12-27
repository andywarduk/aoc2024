use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(2, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    input.iter().filter(|nums| is_safe(nums)).count()
}

fn part2(input: &[InputEnt]) -> usize {
    input.iter().filter(|nums| is_tolerable(nums)).count()
}

fn is_safe(nums: &[i8]) -> bool {
    // Check strictly monotonic increasing or decreasing with max 3 gap
    nums.is_sorted_by(|a, b| (1..=3).contains(&(b - a)))
        || nums.is_sorted_by(|a, b| (1..=3).contains(&(a - b)))
}

fn is_tolerable(nums: &[i8]) -> bool {
    (0..nums.len()).any(|i| {
        is_safe(
            &(nums
                .iter()
                .enumerate()
                .filter_map(|(idx, n)| if idx != i { Some(*n) } else { None })
                .collect::<Vec<_>>()),
        )
    })
}

// Input parsing

type InputEnt = Vec<i8>;

fn input_transform(line: &str) -> InputEnt {
    line.split_ascii_whitespace()
        .map(|s| {
            s.parse::<i8>()
                .unwrap_or_else(|_| panic!("{s} is not an integer"))
        })
        .collect()
}

#[cfg(test)]
mod tests;
