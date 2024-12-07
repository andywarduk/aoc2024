use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(7, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .map(|e| if solveable(e, false) { e.answer } else { 0 })
        .sum::<u64>()
}

fn part2(input: &[Equation]) -> u64 {
    input
        .iter()
        .map(|e| if solveable(e, true) { e.answer } else { 0 })
        .sum::<u64>()
}

fn solveable(e: &Equation, try_concat: bool) -> bool {
    solveable_iter(1, e.values[0], e, try_concat)
}

fn solveable_iter(idx: usize, res: u64, e: &Equation, try_concat: bool) -> bool {
    if idx == e.values.len() {
        return res == e.answer;
    }

    let next = res + e.values[idx];

    if next <= e.answer && solveable_iter(idx + 1, next, e, try_concat) {
        return true;
    }

    let next = res * e.values[idx];

    if next <= e.answer && solveable_iter(idx + 1, next, e, try_concat) {
        return true;
    }

    if try_concat {
        let mut next_str = res.to_string();
        next_str.push_str(&e.values[idx].to_string());
        let next = next_str.parse::<u64>().expect("Unable to parse");

        if next <= e.answer && solveable_iter(idx + 1, next, e, try_concat) {
            return true;
        }
    }

    false
}

type EqnNum = u64;

struct Equation {
    answer: EqnNum,
    values: Vec<EqnNum>,
}

// Input parsing

fn input_transform(line: String) -> Equation {
    let mut s = line.split(':');

    let answer = s
        .next()
        .expect("Answer not found")
        .parse::<EqnNum>()
        .expect("Answer not valid");

    let values = s
        .next()
        .expect("Values not found")
        .trim_ascii_start()
        .split_ascii_whitespace()
        .map(|v| v.parse::<EqnNum>().expect("Value not valid"))
        .collect::<Vec<_>>();

    Equation { answer, values }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 3749);
        assert_eq!(part2(&input), 11387);
    }
}
