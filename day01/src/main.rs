use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(1, input_transform)?;
    let (v1, v2) = split_input(&input);

    // Run parts
    println!("Part 1: {}", part1(&v1, &v2));
    println!("Part 2: {}", part2(&v1, &v2));

    Ok(())
}

fn part1(v1: &[u64], v2: &[u64]) -> u64 {
    v1.iter().zip(v2).map(|(n1, n2)| n1.abs_diff(*n2)).sum()
}

fn part2(v1: &[u64], v2: &[u64]) -> u64 {
    v1.iter()
        .map(|n1| {
            let p1 = v2.partition_point(|n2| n2 < n1);
            v2[p1..].partition_point(|n2| n2 <= n1) as u64 * n1
        })
        .sum()
}

// Input parsing

type InputEnt = Vec<u64>;

fn input_transform(line: String) -> InputEnt {
    line.split_ascii_whitespace()
        .map(|n| n.parse::<u64>().expect("{n} is not an integer"))
        .collect()
}

fn split_input(input: &[Vec<u64>]) -> (Vec<u64>, Vec<u64>) {
    let (mut v1, mut v2) = input
        .iter()
        .fold((Vec::new(), Vec::new()), |(mut v1, mut v2), v| {
            v1.push(v[0]);
            v2.push(v[1]);

            (v1, v2)
        });

    v1.sort();
    v2.sort();

    (v1, v2)
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let (v1, v2) = split_input(&input);

        assert_eq!(part1(&v1, &v2), 11);
        assert_eq!(part2(&v1, &v2), 31);
    }
}
