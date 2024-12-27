use std::error::Error;

use aoc::input::parse_input;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input(13, parse_input_str)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Claw]) -> u64 {
    input
        .iter()
        .filter_map(|c| presses(c, 0))
        .map(|(apresses, bpresses)| (apresses * 3) + bpresses)
        .sum()
}

fn part2(input: &[Claw]) -> u64 {
    input
        .iter()
        .filter_map(|c| presses(c, 10000000000000))
        .map(|(apresses, bpresses)| (apresses * 3) + bpresses)
        .sum()
}

fn presses(c: &Claw, adjust: u64) -> Option<(u64, u64)> {
    // Find line intersection
    let ax = c.a.0 as f64;
    let ay = c.a.1 as f64;

    let bx = c.b.0 as f64;
    let by = c.b.1 as f64;

    let tx = (c.target.0 + adjust) as f64;
    let ty = (c.target.1 + adjust) as f64;

    let denom = (ax * by) - (ay * bx);

    let apresses = (tx * by - ty * bx) / denom;
    let bpresses = (ty * ax - tx * ay) / denom;

    if apresses.fract() != 0.0 || bpresses.fract() != 0.0 {
        None
    } else {
        Some((apresses as u64, bpresses as u64))
    }
}

// Input parsing

type Coord = (u64, u64);

#[derive(Debug)]
struct Claw {
    a: Coord,
    b: Coord,
    target: Coord,
}

fn parse_input_str(file: &str) -> Vec<Claw> {
    let re = Regex::new(r"\d+").expect("Failed to create regex");

    file.split("\n\n")
        .map(|chunk| {
            let mut tuples = chunk.lines().map(|line| {
                let mut captures = re
                    .find_iter(line)
                    .map(|c| c.as_str().parse::<u64>().unwrap());

                (
                    captures.next().expect("First u64 not present"),
                    captures.next().expect("Second u64 not present"),
                )
            });

            Claw {
                a: tuples.next().expect("First tuple not present"),
                b: tuples.next().expect("Second tuple not present"),
                target: tuples.next().expect("Third tuple not present"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
