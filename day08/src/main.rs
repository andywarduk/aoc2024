use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(8, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    let positions = get_positions(input);

    let mut intpos = HashSet::new();

    let mut add_pos = |x, y, ox, oy| {
        if x >= 0
            && y >= 0
            && x < input[0].len() as isize
            && y < input.len() as isize
            && (x, y) != (ox, oy)
        {
            intpos.insert((x, y));
        }
    };

    for (_c, p) in positions {
        for (i, (x1, y1)) in p.iter().enumerate() {
            let (x1, y1) = (*x1 as isize, *y1 as isize);

            for (x2, y2) in p[i + 1..].iter() {
                let (x2, y2) = (*x2 as isize, *y2 as isize);

                let (xd, yd) = (x2 - x1, y2 - y1);

                add_pos(x1 - xd, y1 - yd, x2, y2);
                add_pos(x1 + xd, y1 + yd, x2, y2);

                add_pos(x2 - xd, y2 - yd, x1, y1);
                add_pos(x2 + xd, y2 + yd, x1, y1);
            }
        }
    }

    intpos.len() as u64
}

fn part2(input: &[InputEnt]) -> u64 {
    let positions = get_positions(input);

    let mut intpos = HashSet::new();

    let mut add_pos = |x, y, xd, yd| {
        let mut x = x as isize;
        let mut y = y as isize;

        loop {
            intpos.insert((x as usize, y as usize));

            x += xd;
            y += yd;

            if x < 0 || x as usize >= input[0].len() || y < 0 || y as usize >= input.len() {
                break;
            }
        }
    };

    for (_c, p) in positions {
        for (i, (x1, y1)) in p.iter().enumerate() {
            for (x2, y2) in p[i + 1..].iter() {
                let xd = *x2 as isize - *x1 as isize;
                let yd = *y2 as isize - *y1 as isize;

                add_pos(*x1, *y1, -xd, -yd);
                add_pos(*x1, *y1, xd, yd);
            }
        }
    }

    intpos.len() as u64
}

// Input parsing

type InputEnt = Vec<char>;

fn input_transform(line: String) -> InputEnt {
    line.chars().collect()
}

fn get_positions(input: &[InputEnt]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != '.' {
                positions
                    .entry(*c)
                    .and_modify(|v| v.push((x, y)))
                    .or_insert(vec![(x, y)]);
            }
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 14);
        assert_eq!(part2(&input), 34);
    }
}