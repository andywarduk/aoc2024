use std::{collections::HashSet, error::Error, sync::LazyLock};

use aoc::{gif::Gif, input::parse_input_vec};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(14, input_transform)?;

    // Count secs
    let board = Board::new(101, 103, &input);
    let secs = count(board);

    // Draw
    let board = Board::new(101, 103, &input);
    draw(board, secs)?;

    Ok(())
}

fn count(mut board: Board) -> usize {
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

const SCALE: u16 = 8;

fn draw(mut board: Board, secs: usize) -> Result<(), Box<dyn Error>> {
    let palette = vec![[0, 0, 0], [0, 255, 0]];

    let mut gif = Gif::new(
        "vis/day14.gif",
        &palette,
        board.w as u16,
        board.h as u16,
        SCALE,
        SCALE,
    )?;

    draw_board(&mut gif, &board)?;

    let draw_from = secs - 100;

    for s in 0..secs {
        board.step();

        if s > draw_from {
            draw_board(&mut gif, &board)?;
        }
    }

    gif.delay(500)?;

    Ok(())
}

fn draw_board(gif: &mut Gif, board: &Board) -> Result<(), Box<dyn Error>> {
    let mut frame = gif.empty_frame();

    for r in &board.robots {
        frame[r.y][r.x] = 1;
    }

    gif.draw_frame(frame, 5)?;

    Ok(())
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
