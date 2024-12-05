use std::{collections::HashMap, error::Error};

use aoc::input::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(5)?;
    let (orders, prints) = parse_input(&input);

    // Run parts
    println!("Part 1: {}", part1(&orders, &prints));
    println!("Part 2: {}", part2(&orders, &prints));

    Ok(())
}

fn part1(orders: &Vec<Vec<u8>>, prints: &Vec<Vec<u8>>) -> u64 {
    let mut sum: u64 = 0;
    let mut map: HashMap<u8, Vec<u8>> = HashMap::new();

    for o in orders {
        map.entry(o[0])
            .and_modify(|e| e.push(o[1]))
            .or_insert(vec![o[1]]);
    }

    for p in prints {
        let mut ok = true;

        'failed: for (i, n) in p.iter().enumerate().skip(1) {
            if let Some(v) = map.get(n) {
                for n in v {
                    if p[..i].contains(n) {
                        ok = false;
                        break 'failed;
                    }
                }
            }
        }

        if ok {
            sum += p[p.len() / 2] as u64;
        }
    }

    sum
}

fn part2(orders: &Vec<Vec<u8>>, prints: &Vec<Vec<u8>>) -> u64 {
    let mut sum: u64 = 0;
    let mut map: HashMap<u8, Vec<u8>> = HashMap::new();

    for o in orders {
        map.entry(o[0])
            .and_modify(|e| e.push(o[1]))
            .or_insert(vec![o[1]]);
    }

    for p in prints {
        let mut ok = true;

        'failed: for (i, n) in p.iter().enumerate().skip(1) {
            if let Some(v) = map.get(n) {
                for n in v {
                    if p[..i].contains(n) {
                        ok = false;
                        break 'failed;
                    }
                }
            }
        }

        if !ok {
            let o = find_order(p, &map);
            sum += o[o.len() / 2] as u64;
        }
    }

    sum
}

fn find_order(p: &[u8], map: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    for start in p {
        if let Some(o) = find_order_rec(&[*start], p, map) {
            return o;
        }
    }

    panic!("No solution found")
}

fn find_order_rec(o: &[u8], p: &[u8], map: &HashMap<u8, Vec<u8>>) -> Option<Vec<u8>> {
    if let Some(v) = map.get(&o[o.len() - 1]) {
        for n in v {
            if p.contains(n) && !o.contains(n) {
                let next = [o, &[*n]].concat();

                if next.len() == p.len() {
                    println!("Found {next:?}");
                    return Some(next);
                } else if let Some(o) = find_order_rec(&[o, &[*n]].concat(), p, map) {
                    return Some(o);
                }
            }
        }
    }

    None
}

// Input parsing

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut sections = input.split("\n\n");

    let section = sections.next().expect("Section 1 not found");

    let orders = section
        .lines()
        .map(|l| {
            l.split("|")
                .map(|n| n.parse::<u8>().expect("Error parsing u8"))
                .collect()
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
