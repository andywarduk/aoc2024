#![feature(portable_simd)]

use std::error::Error;
use std::simd::prelude::*;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(22, |line| line.parse::<u64>().unwrap())?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

const ITERS: usize = 2000;

fn part1(input: &[u64]) -> u64 {
    input
        .iter()
        .map(|line| {
            let mut secret = *line;

            // Do hash iterations
            for _ in 0..ITERS {
                hashstep(&mut secret);
            }

            secret
        })
        .sum()
}

const RANGE: usize = 19;
const RANGEP2: usize = RANGE.pow(2);
const RANGEP3: usize = RANGE.pow(3);
const RANGEP4: usize = RANGE.pow(4);

const MULT: Simd<u16, 4> = u16x4::from_array([1, RANGE as u16, RANGEP2 as u16, RANGEP3 as u16]);

fn part2(input: &[u64]) -> u64 {
    // Map 4 price changes to total number of bananas
    let mut set = [false; RANGEP4];
    let mut bananas = [0u16; RANGEP4];

    for line in input {
        // Calculate 2000 prices
        let mut secret = *line;

        let prices = (0..ITERS)
            .map(|_| {
                hashstep(&mut secret);
                (secret % 10) as u8
            })
            .collect::<Vec<_>>();

        // Initialise set flags
        set.fill(false);

        // Calculate the price changes
        let diffs = prices
            .windows(2)
            .map(|a| ((a[1] as i8 - a[0] as i8) + 9) as u16) // range 0-18
            .collect::<Vec<_>>();

        // Map windows of 4 price changes
        diffs
            .windows(4)
            .map(|arr| {
                // ... build array element
                let diffs = u16x4::from_slice(arr);
                let diffs_mult = diffs * MULT;
                diffs_mult.reduce_sum() as usize
            })
            .enumerate()
            .for_each(|(i, elem)| {
                // Already set?
                if !set[elem] {
                    // No - accumulate
                    bananas[elem] += prices[i + 4] as u16;
                    set[elem] = true;
                }
            });
    }

    // Get the max number of bananas possible
    let max = bananas.iter().max().unwrap();

    *max as u64
}

fn hashstep(secret: &mut u64) {
    *secret ^= *secret << 6;
    *secret &= 0xffffff;

    *secret ^= *secret >> 5;

    *secret ^= *secret << 11;
    *secret &= 0xffffff;
}

#[cfg(test)]
mod tests;
