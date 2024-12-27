use std::{error::Error, ops::Range, sync::LazyLock};

use aoc::input::parse_input_vec;
use fxhash::FxHashSet;
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
        let mut set = FxHashSet::default();

        for r in &self.robots {
            if !set.insert((r.x, r.y)) {
                return false;
            }
        }

        true
    }
}

// Input parsing

type InputEnt = Robot;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

fn input_transform(line: &str) -> InputEnt {
    let c: [&str; 4] = RE
        .captures(line)
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
mod tests;
