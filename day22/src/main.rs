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

fn part1(input: &[u64]) -> u64 {
    input
        .iter()
        .map(|line| {
            let mut secret = *line;

            // Do 2000 hash iterations
            for _ in 0..2000 {
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

        let prices = (0..2000)
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
        let mut lhashmap: FxHashMap<[i8; 4], u64> = FxHashMap::default();
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
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254
";

    #[test]
    fn test1() {
        let mut secret = 123;

        let next = EXAMPLE1
            .lines()
            .map(|l| l.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        for n in next {
            hashstep(&mut secret);
            assert_eq!(secret, n);
        }
    }

    const EXAMPLE2: &str = "\
1: 8685429
10: 4700978
100: 15273692
2024: 8667524
";

    #[test]
    fn test2() {
        let sec_result = EXAMPLE2
            .lines()
            .map(|l| {
                let mut parts = l.split_ascii_whitespace();

                let secret = parts
                    .next()
                    .unwrap()
                    .trim_end_matches(':')
                    .parse::<u64>()
                    .unwrap();
                let result = parts.next().unwrap().parse::<u64>().unwrap();

                (secret, result)
            })
            .collect::<Vec<_>>();

        let sum = sec_result
            .into_iter()
            .map(|(mut secret, result)| {
                for _ in 0..2000 {
                    hashstep(&mut secret);
                }

                assert_eq!(secret, result);

                secret
            })
            .sum::<u64>();

        assert_eq!(sum, 37327623);
    }

    const EXAMPLE3: &str = "\
1
2
3
2024
";

    #[test]
    fn test3() {
        let input = parse_test_vec(EXAMPLE3, input_transform).unwrap();
        assert_eq!(part2(&input), 23);
    }
}
