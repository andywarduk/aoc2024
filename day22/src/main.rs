use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashMap;

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
    let mut diffmap: FxHashMap<[i8; 4], u64> = FxHashMap::default();

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
            .map(|a| a[1] as i8 - a[0] as i8)
            .collect::<Vec<_>>();

        // Build map of 4 price changes to number of bananas
        let mut lhashmap: FxHashMap<[i8; 4], u64> =
            FxHashMap::with_capacity_and_hasher(diffs.len(), Default::default());
        let mut diffcpy: [i8; 4] = [0; 4];

        for dn in 0..(diffs.len() - 3) {
            diffcpy.copy_from_slice(&diffs[dn..(dn + 4)]);

            lhashmap.entry(diffcpy).or_insert_with(|| prices[dn + 4]);
        }

        // Update the total number of bananas
        for (diffs, bananas) in lhashmap {
            diffmap
                .entry(diffs)
                .and_modify(|e| *e += bananas)
                .or_insert(bananas);
        }
    }

    // Get the max number of bananas possible
    let max = diffmap.values().max().copied().unwrap();

    max
}

fn hashstep(secret: &mut u64) {
    let calc1 = *secret * 64;
    *secret ^= calc1;
    *secret %= 16777216;

    let calc2 = *secret / 32;
    *secret ^= calc2;
    *secret %= 16777216;

    let calc3 = *secret * 2048;
    *secret ^= calc3;
    *secret %= 16777216;
}

// Input parsing

fn input_transform(line: String) -> u64 {
    line.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests;
