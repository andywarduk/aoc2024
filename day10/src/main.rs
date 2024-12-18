use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(10, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

type Coord = (usize, usize);

fn part1(input: &[InputEnt]) -> u64 {
    heads(input)
        .map(|(x, y)| {
            let mut dests = FxHashSet::default();

            walk1(input, x, y, 1, &mut dests);

            dests.len() as u64
        })
        .sum()
}

fn walk1(input: &[InputEnt], x: usize, y: usize, h: u8, dests: &mut FxHashSet<Coord>) {
    pos_from(input, x, y, h).for_each(|(nx, ny)| {
        if h == 9 {
            dests.insert((nx, ny));
        } else {
            walk1(input, nx, ny, h + 1, dests);
        }
    })
}

fn part2(input: &[InputEnt]) -> u64 {
    heads(input).map(|(x, y)| walk2(input, x, y, 1)).sum()
}

fn walk2(input: &[InputEnt], x: usize, y: usize, h: u8) -> u64 {
    pos_from(input, x, y, h)
        .map(|(nx, ny)| {
            if h == 9 {
                1
            } else {
                walk2(input, nx, ny, h + 1)
            }
        })
        .sum()
}

fn heads(input: &[InputEnt]) -> impl Iterator<Item = Coord> {
    input.iter().enumerate().flat_map(|(y, l)| {
        l.iter()
            .enumerate()
            .filter_map(move |(x, h)| if *h == 0 { Some((x, y)) } else { None })
    })
}

const DIRS: [[isize; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn pos_from(input: &[InputEnt], x: usize, y: usize, h: u8) -> impl Iterator<Item = Coord> {
    DIRS.into_iter().filter_map(move |[dx, dy]| {
        match x.checked_add_signed(dx) {
            Some(nx) if nx < input[0].len() => match y.checked_add_signed(dy) {
                Some(ny) if ny < input.len() => {
                    if input[ny][nx] == h {
                        return Some((nx, ny));
                    }
                }
                _ => (),
            },
            _ => (),
        }

        None
    })
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: String) -> InputEnt {
    line.chars().map(|c| c as u8 - b'0').collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 36);
        assert_eq!(part2(&input), 81);
    }
}
