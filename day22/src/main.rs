#![feature(array_windows)]

use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::{FxHashMap, FxHashSet};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(22, input_transform)?;

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

fn part2(input: &[u64]) -> u64 {
    // Map 4 price changes to total number of bananas
    let mut diffmap: FxHashMap<u32, u64> = FxHashMap::default();
    let mut lhashset: FxHashSet<u32> = FxHashSet::default();

    for line in input {
        // Calculate 2000 prices
        let mut secret = *line;

        let prices = (0..ITERS)
            .map(|_| {
                hashstep(&mut secret);
                secret % 10
            })
            .collect::<Vec<_>>();

        // Calculate the price changes
        let diffs = prices
            .windows(2)
            .map(|a| (a[1] as i8 - a[0] as i8) as u8)
            .collect::<Vec<_>>();

        // Clear loop hash set
        lhashset.clear();

        // Iterate set of 4 price changes for this input line
        diffs
            .array_windows::<4>()
            .zip(prices.iter().skip(4))
            .for_each(|(set, price)| {
                // Build u32 from 4 u8s
                let key = u32::from_ne_bytes(*set);

                // Already got this set of diffs?
                if lhashset.insert(key) {
                    // No - update the total number of bananas
                    *diffmap.entry(key).or_insert(0) += *price;
                }
            });
    }

    // Get the max number of bananas possible
    let max = diffmap.values().max().copied().unwrap();

    max
}

fn hashstep(secret: &mut u64) {
    *secret ^= *secret << 6;
    *secret &= 0xffffff;

    *secret ^= *secret >> 5;
    *secret &= 0xffffff;

    *secret ^= *secret << 11;
    *secret &= 0xffffff;
}

// Input parsing

fn input_transform(line: String) -> u64 {
    line.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests;
