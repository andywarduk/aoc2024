use std::error::Error;

use aoc::input::read_input_file;
use fxhash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let (available, designs) = parse_input();

    let composable = build_composable(&available, &designs);

    // Run parts
    println!("Part 1: {}", part1(&composable));
    println!("Part 2: {}", part2(&composable));

    Ok(())
}

fn part1(composable: &[usize]) -> u64 {
    composable.iter().filter(|c| **c != 0).count() as u64
}

fn part2(composable: &[usize]) -> u64 {
    composable.iter().sum::<usize>() as u64
}

fn build_composable(available: &FxHashSet<String>, designs: &[String]) -> Vec<usize> {
    let maxlen = available.iter().map(|a| a.len()).max().unwrap();

    designs
        .iter()
        .map(|design| {
            let dlen = design.len();
            let mut composable = vec![0usize; dlen];

            for idx in (0..dlen).rev() {
                let scan = maxlen.min(dlen - idx);

                for end in 1..=scan {
                    let chunk = &design[idx..idx + end];

                    if let Some(a) = available.get(chunk) {
                        let next_idx = idx + a.len();

                        if next_idx == dlen {
                            composable[idx] += 1;
                        } else {
                            composable[idx] += composable[next_idx];
                        }
                    }
                }
            }

            composable[0]
        })
        .collect()
}

// Input parsing

fn parse_input() -> (FxHashSet<String>, Vec<String>) {
    let input = read_input_file(19).unwrap();

    parse_input_string(&input)
}

fn parse_input_string(input: &str) -> (FxHashSet<String>, Vec<String>) {
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
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test1() {
        let (available, designs) = parse_input_string(EXAMPLE1);

        let composable = build_composable(&available, &designs);

        assert_eq!(part1(&composable), 6);
        assert_eq!(part2(&composable), 16);
    }
}
