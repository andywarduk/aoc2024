use std::{cmp::Ordering, error::Error};

use aoc::input::read_input_file;
use fxhash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(5)?;
    let (orders, prints) = parse_input(&input);

    // Run parts
    println!("Part 1: {}", part1(&orders, &prints));
    println!("Part 2: {}", part2(&orders, &prints));

    Ok(())
}

fn part1(orders: &PageOrder, prints: &[Vec<u8>]) -> u64 {
    prints
        .iter()
        .filter_map(|print| match correct_order(print, orders) {
            None => Some(print[print.len() / 2] as u64),
            _ => None,
        })
        .sum()
}

fn part2(orders: &PageOrder, prints: &[Vec<u8>]) -> u64 {
    prints
        .iter()
        .filter_map(|print| correct_order(print, orders).map(|order| order[order.len() / 2] as u64))
        .sum()
}

fn correct_order(print: &[u8], orders: &PageOrder) -> Option<Vec<u8>> {
    let mut sorted = print.to_vec();

    sorted.sort_by(|a, b| {
        if orders.contains(&[*a, *b]) {
            Ordering::Less
        } else if orders.contains(&[*b, *a]) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    if sorted != *print { Some(sorted) } else { None }
}

// Input parsing

type PageOrder = FxHashSet<[u8; 2]>;

fn parse_input(input: &str) -> (PageOrder, Vec<Vec<u8>>) {
    let mut sections = input.split("\n\n");

    let section = sections.next().expect("Section 1 not found");

    let orders = section
        .lines()
        .map(|l| {
            let mut s = l.split("|");

            [
                s.next()
                    .expect("First u8 not found")
                    .parse::<u8>()
                    .expect("Error parsing first u8"),
                s.next()
                    .expect("Second u8 not found")
                    .parse::<u8>()
                    .expect("Error parsing second u8"),
            ]
        })
        .collect();

    let section = sections.next().expect("Section 2 not found");

    let prints = section
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<u8>().expect("Error parsing u8"))
                .collect()
        })
        .collect();

    (orders, prints)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test1() {
        let (orders, prints) = parse_input(EXAMPLE1);
        assert_eq!(part1(&orders, &prints), 143);
        assert_eq!(part2(&orders, &prints), 123);
    }
}
