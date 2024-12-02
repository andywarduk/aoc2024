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
    input.iter().filter(|&nums| is_safe(nums)).count()
}

fn part2(input: &[InputEnt]) -> usize {
    input
        .iter()
        .filter(|&nums| {
            if is_safe(nums) {
                true
            } else {
                // Try with a number removed
                for i in 0..nums.len() {
                    let mut nums2 = nums.clone();
                    nums2.remove(i);

                    if is_safe(&nums2) {
                        return true;
                    }
                }

                false
            }
        })
        .count()
}

fn is_safe(nums: &[u8]) -> bool {
    let mut num_iter = nums.iter();
    let mut increasing = None;
    let mut last = num_iter.next().expect("Failed to get first number");

    for n in num_iter {
        match increasing {
            None => {
                if n == last {
                    return false;
                } else {
                    increasing = Some(n > last);
                }
            }
            Some(true) => {
                if n <= last {
                    return false;
                }
            }
            Some(false) => {
                if n >= last {
                    return false;
                }
            }
        }

        if n.abs_diff(*last) > 3 {
            return false;
        }

        last = n;
    }

    true
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: String) -> InputEnt {
    line.split_ascii_whitespace()
        .map(|s| {
            s.parse::<u8>()
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
