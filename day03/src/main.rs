use regex::Regex;
use std::error::Error;

use aoc::input::parse_input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input(3, |s| s.to_string())?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    let re = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").expect("Failed to create regex");

    re.captures_iter(input)
        .map(|nums| nums.extract())
        .map(|(_instr, [astr, bstr])| {
            let a = astr.parse::<u64>().expect("a is not u64");
            let b = bstr.parse::<u64>().expect("b is not u64");

            a * b
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut pos = 0;
    let mut filtered = String::with_capacity(input.len());

    loop {
        match input[pos..].find("don't()") {
            Some(p) => {
                filtered.push_str(&input[pos..(pos + p)]);
                pos += p;
            }
            None => {
                filtered.push_str(&input[pos..]);
                break;
            }
        }

        match input[pos..].find("do()") {
            Some(p) => {
                pos += p + 4;
            }
            None => {
                break;
            }
        }
    }

    part1(&filtered)
}

#[cfg(test)]
mod tests;
