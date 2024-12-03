use regex::Regex;
use std::error::Error;

use aoc::input::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(3)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    let re = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").expect("Failed to create regex");

    re.captures_iter(input)
        .map(|nums| nums.extract())
        .map(|(_instr, [astr, bstr])| {
            let a = astr.parse::<u64>().expect("a is not u64");
            let b = bstr.parse::<u64>().expect("b is not u64");

            a * b
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut pos = 0;
    let mut filtered = String::with_capacity(input.len());

    loop {
        match input[pos..].find("don't()") {
            Some(p) => {
                filtered.push_str(&input[pos..(pos + p)]);
                pos += p;
            }
            None => {
                filtered.push_str(&input[pos..]);
                break;
            }
        }

        match input[pos..].find("do()") {
            Some(p) => {
                pos += p + 4;
            }
            None => {
                break;
            }
        }
    }

    part1(&filtered)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test1() {
        assert_eq!(part1(EXAMPLE1), 161);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(EXAMPLE2), 48);
    }
}
