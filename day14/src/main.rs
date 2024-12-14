use std::{collections::HashSet, error::Error, fmt, ops::Range, sync::LazyLock};

use aoc::input::parse_input_vec;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(14, input_transform)?;

    // Run parts
    let board = Board::new(101, 103, &input);
    println!("Part 1: {}", part1(board));

    let board = Board::new(101, 103, &input);
    println!("Part 2: {}", part2(board));

    Ok(())
}

fn part1(mut board: Board) -> u64 {
    for _ in 0..100 {
        board.step();
    }

    let qx = board.w / 2;
    let qy = board.h / 2;

    let mut qr = [0; 4];

    let mut check = |r: &Robot, i, xr: Range<usize>, yr: Range<usize>| {
        if xr.contains(&r.x) && yr.contains(&r.y) {
            qr[i] += 1;
        }
    };

    for r in board.robots.iter() {
        check(r, 0, 0..qx, 0..qy);
        check(r, 1, board.w - qx..board.w, 0..qy);
        check(r, 2, 0..qx, board.h - qy..board.h);
        check(r, 3, board.w - qx..board.w, board.h - qy..board.h);
    }

    qr.iter().product::<u64>()
}

fn part2(mut board: Board) -> u64 {
    let mut secs = 0;

    loop {
        board.step();
        secs += 1;

        if board.interesting() {
            println!("{board}");
            break;
        }
    }

    secs
}

#[derive(Debug, Clone)]
struct Robot {
    x: usize,
    y: usize,
    vx: isize,
    vy: isize,
}

struct Board {
    w: usize,
    h: usize,
    robots: Vec<Robot>,
}

impl Board {
    fn new(w: usize, h: usize, robots: &[Robot]) -> Self {
        Self {
            w,
            h,
            robots: robots.to_vec(),
        }
    }

    fn step(&mut self) {
        self.robots.iter_mut().for_each(|r| {
            r.x = ((r.x as isize + r.vx).rem_euclid(self.w as isize)) as usize;
            r.y = ((r.y as isize + r.vy).rem_euclid(self.h as isize)) as usize;
        })
    }

    fn interesting(&self) -> bool {
        // No overlaps
        let mut set = HashSet::new();

        for r in &self.robots {
            if !set.insert((r.x, r.y)) {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                let r = self
                    .robots
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.x == x && r.y == y)
                    .collect::<Vec<_>>();

                if r.is_empty() {
                    write!(f, ".")?;
                } else if r.len() == 1 {
                    write!(f, "{}", (b'A' + (r[0].0 % 26) as u8) as char)?;
                } else {
                    write!(f, "{}", r.len())?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// Input parsing

type InputEnt = Robot;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

fn input_transform(line: String) -> InputEnt {
    let c: [&str; 4] = RE
        .captures(&line)
        .map(|c| c.extract())
        .map(|(_, arr)| arr)
        .expect("Pattern not found");

    Robot {
        x: c[0].parse::<usize>().unwrap(),
        y: c[1].parse::<usize>().unwrap(),
        vx: c[2].parse::<isize>().unwrap(),
        vy: c[3].parse::<isize>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        let board = Board::new(11, 7, &input);
        assert_eq!(part1(board), 12);
    }
}
