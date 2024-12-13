use std::error::Error;

use aoc::input::read_input_file;
use regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = get_input()?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Claw]) -> u64 {
    input
        .iter()
        .map(|c| match presses(c, (c.target.0, c.target.1)) {
            Some((apresses, bpresses)) => (apresses * 3) + bpresses,
            None => 0,
        })
        .sum()
}

fn part2(input: &[Claw]) -> u64 {
    input
        .iter()
        .map(|c| {
            match presses(
                c,
                (c.target.0 + 10000000000000, c.target.1 + 10000000000000),
            ) {
                Some((apresses, bpresses)) => (apresses * 3) + bpresses,
                None => 0,
            }
        })
        .sum()
}

fn presses(c: &Claw, target: Coord) -> Option<(u64, u64)> {
    // Find line intersection
    let x1 = dec!(0);
    let y1 = dec!(0);
    let x2 = Decimal::from(c.a.0);
    let y2 = Decimal::from(c.a.1);
    let x3 = Decimal::from(target.0);
    let y3 = Decimal::from(target.1);
    let x4 = Decimal::from(target.0 + c.b.0);
    let y4 = Decimal::from(target.1 + c.b.1);

    let denom = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));

    let ix =
        ((((x1 * y2) - (y1 * x2)) * (x3 - x4)) - ((x1 - x2) * ((x3 * y4) - (y3 * x4)))) / denom;
    let iy =
        ((((x1 * y2) - (y1 * x2)) * (y3 - y4)) - ((y1 - y2) * ((x3 * y4) - (y3 * x4)))) / denom;

    if ix < dec!(0) || ix.fract() != dec!(0) || iy < dec!(0) || iy.fract() != dec!(0) {
        return None;
    }

    let apresses = ix / x2;
    let bpresses = (x3 - ix) / Decimal::from(c.b.0);

    if apresses.fract() != dec!(0) || bpresses.fract() != dec!(0) {
        return None;
    }

    let apresses = apresses.to_u64().unwrap();
    let bpresses = bpresses.to_u64().unwrap();

    Some((apresses, bpresses))
}

// Input parsing

type Coord = (u64, u64);

#[derive(Debug)]
struct Claw {
    a: Coord,
    b: Coord,
    target: Coord,
}

fn get_input() -> Result<Vec<Claw>, Box<dyn Error>> {
    let file = read_input_file(13)?;

    Ok(parse_input(&file))
}

fn parse_input(file: &str) -> Vec<Claw> {
    let re1 = Regex::new("X\\+([0-9]+), Y\\+([0-9]+)").expect("Failed to create regex");
    let re2 = Regex::new("X=([0-9]+), Y=([0-9]+)").expect("Failed to create regex");

    let get_tuple = |re: &Regex, line| {
        let captures = re.captures(line).expect("re failed");

        let x = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let y = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();

        (x, y)
    };

    file.split("\n\n")
        .map(|chunk| {
            let mut lines = chunk.lines();

            let a = get_tuple(&re1, lines.next().expect("No line A"));
            let b = get_tuple(&re1, lines.next().expect("No line B"));
            let target = get_tuple(&re2, lines.next().expect("No prize line"));

            Claw { a, b, target }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test1() {
        let input = parse_input(EXAMPLE1);
        assert_eq!(part1(&input), 480);
        assert_eq!(part2(&input), 0 /* TODO */);
    }
}
