use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(7, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Equation]) -> u64 {
    solveable_sum(input, false)
}

fn part2(input: &[Equation]) -> u64 {
    solveable_sum(input, true)
}

fn solveable_sum(input: &[Equation], try_concat: bool) -> u64 {
    input.iter().fold(0, |acc, e| {
        if solveable(e, try_concat) {
            acc + e.answer
        } else {
            acc
        }
    })
}

fn solveable(e: &Equation, try_concat: bool) -> bool {
    solveable_iter(1, e.values[0], e, try_concat)
}

fn solveable_iter(idx: usize, res: u64, e: &Equation, try_concat: bool) -> bool {
    // Any more values?
    if idx == e.values.len() {
        // No - check against answer
        return res == e.answer;
    }

    // Try adding first
    let next = res + e.values[idx];

    if next <= e.answer && solveable_iter(idx + 1, next, e, try_concat) {
        return true;
    }

    // Try multiplication
    let next = res * e.values[idx];

    if next <= e.answer && solveable_iter(idx + 1, next, e, try_concat) {
        return true;
    }

    // Try concatenating the digits
    if try_concat {
        // Count digits in the next number
        let digits = 1 + e.values[idx].ilog10();

        // Multiply by 10^digits and add
        let next = (res * 10u64.pow(digits)) + e.values[idx];

        if next <= e.answer && solveable_iter(idx + 1, next, e, try_concat) {
            return true;
        }
    }

    false
}

struct Equation {
    answer: u64,
    values: Vec<u64>,
}

// Input parsing

fn input_transform(line: String) -> Equation {
    let mut s = line.split(':');

    let answer = s
        .next()
        .expect("Answer not found")
        .parse::<u64>()
        .expect("Answer not valid");

    let values = s
        .next()
        .expect("Values not found")
        .trim_ascii_start()
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().expect("Value not valid"))
        .collect::<Vec<_>>();

    Equation { answer, values }
}

#[cfg(test)]
mod tests;
