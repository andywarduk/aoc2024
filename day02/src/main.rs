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

fn input_transform(line: String) -> InputEnt {
    line.split_ascii_whitespace()
        .map(|s| {
            s.parse::<i8>()
                .unwrap_or_else(|_| panic!("{s} is not an integer"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }
}
