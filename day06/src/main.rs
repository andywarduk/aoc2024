use std::{collections::HashSet, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(6, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[BoardLine]) -> u64 {
    let (gx, gy) = guard_pos(input);

    walk_path(input, gx, gy).len() as u64
}

fn part2(input: &[BoardLine]) -> u64 {
    let (gx, gy) = guard_pos(input);

    let mut path = walk_path(input, gx, gy);
    path.remove(&(gx, gy));

    path.iter()
        .filter(|(bx, by)| loop_check(input, gx, gy, *bx, *by))
        .count() as u64
}

fn walk_path(input: &[BoardLine], mut gx: usize, mut gy: usize) -> HashSet<(usize, usize)> {
    let boardx = input[0].len();
    let boardy = input.len();

    let mut dir = Dir::N;

    let mut visited = HashSet::new();
    visited.insert((gx, gy));

    while let Some((nx, ny)) = dir.next_pos(gx, gy, boardx, boardy) {
        if matches!(input[ny][nx], Space::Blocked) {
            dir = dir.rotate();
        } else {
            (gx, gy) = (nx, ny);
            visited.insert((gx, gy));
        }
    }

    visited
}

fn loop_check(input: &[BoardLine], mut gx: usize, mut gy: usize, bx: usize, by: usize) -> bool {
    let boardx = input[0].len();
    let boardy = input.len();

    let mut dir = Dir::N;

    let mut turns = HashSet::new();

    while let Some((nx, ny)) = dir.next_pos(gx, gy, boardx, boardy) {
        if matches!(input[ny][nx], Space::Blocked) || (nx, ny) == (bx, by) {
            dir = dir.rotate();

            let ent = (gx, gy, dir.clone());

            if turns.contains(&ent) {
                return true;
            }

            turns.insert(ent);
        } else {
            (gx, gy) = (nx, ny);
        }
    }

    false
}

fn guard_pos(input: &[BoardLine]) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == Space::Guard { Some(x) } else { None })
                .map(|x| (x, y))
        })
        .expect("Unable to find the guard")
}

#[derive(PartialEq)]
enum Space {
    Blocked,
    Empty,
    Guard,
}

type BoardLine = Vec<Space>;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn next_pos(&self, gx: usize, gy: usize, xdim: usize, ydim: usize) -> Option<(usize, usize)> {
        let (dx, dy) = match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        };

        let move_dir = |p, d, max| match d {
            -1 => {
                if p == 0 {
                    None
                } else {
                    Some(p - 1)
                }
            }
            1 => {
                let p = p + 1;
                if p == max { None } else { Some(p) }
            }
            _ => Some(p),
        };

        Some((move_dir(gx, dx, xdim)?, move_dir(gy, dy, ydim)?))
    }

    fn rotate(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
}

// Input parsing

fn input_transform(line: String) -> BoardLine {
    line.chars()
        .map(|c| match c {
            '.' => Space::Empty,
            '#' => Space::Blocked,
            '^' => Space::Guard,
            _ => panic!("Invalid board char {c}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 41);
        assert_eq!(part2(&input), 6);
    }
}
