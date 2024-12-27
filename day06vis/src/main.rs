use std::error::Error;

use aoc::{gif::Gif, input::parse_input_vec};

const CELLSIZE: usize = 7;

const GUARD: [[[u8; CELLSIZE]; CELLSIZE]; 4] = [
    [
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 1, 1],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
    ],
    [
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 1, 0, 0],
        [1, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 1, 1, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
    ],
    [
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
    ],
    [
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 1],
        [0, 0, 1, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
    ],
];

const VISITED: [[u8; CELLSIZE]; CELLSIZE] = [
    [0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 0],
    [0, 0, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0],
];

const BLOCK: [[u8; CELLSIZE]; CELLSIZE] = [
    [0, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 0],
];

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(6, input_transform)?;

    let mut board = input.clone();

    let boardx = board[0].len();
    let boardy = board.len();

    let (mut gx, mut gy) = guard_pos(&input);
    board[gy][gx] = Space::Guard(Dir::N, 1);

    let mut palette = vec![[0, 0, 0], [64, 64, 255], [255, 128, 255], [0, 0, 0]];

    for i in 0..3 {
        let c = 127 + (i * (128 / 3));
        palette.push([c, c, c]);
    }

    palette.push([255, 0, 0]);

    let mut gif = Gif::new(
        "vis/day06-1.gif",
        &palette,
        (boardx * CELLSIZE) as u16,
        (boardy * CELLSIZE) as u16,
        1,
        1,
    )?;

    loop {
        draw_frame(&mut gif, &board)?;

        let g = board[gy][gx].clone();

        if let Space::Guard(dir, gvisited) = g {
            if let Some((nx, ny)) = dir.next_pos(gx, gy, boardx, boardy) {
                match board[ny][nx] {
                    Space::Blocked => board[gy][gx] = Space::Guard(dir.rotate(), gvisited),
                    Space::Empty(evisited) => {
                        board[gy][gx] = Space::Empty(gvisited);
                        (gx, gy) = (nx, ny);
                        board[gy][gx] = Space::Guard(dir.clone(), evisited + 1);
                    }
                    _ => unreachable!(),
                }
            } else {
                break;
            }
        } else {
            panic!("guard not valid")
        }
    }

    gif.delay(500)?;

    Ok(())
}

fn draw_frame(gif: &mut Gif, board: &Vec<BoardLine>) -> Result<(), Box<dyn Error>> {
    let mut frame = gif.empty_frame();

    let draw_masked =
        |frame: &mut Vec<Vec<u8>>, x: usize, y: usize, c: u8, mask: &[[u8; CELLSIZE]; CELLSIZE]| {
            for dx in 0..CELLSIZE {
                for dy in 0..CELLSIZE {
                    if mask[dy][dx] != 0 {
                        frame[y + dy][x + dx] = c;
                    }
                }
            }
        };

    let mut gy = 0;

    for l in board {
        let mut gx = 0;

        for c in l {
            match c {
                Space::Blocked => draw_masked(&mut frame, gx, gy, 1, &BLOCK),
                Space::Empty(visited) => draw_masked(&mut frame, gx, gy, 3 + visited, &VISITED),
                Space::Guard(dir, _) => {
                    let gpic = &GUARD[match dir {
                        Dir::N => 0,
                        Dir::E => 1,
                        Dir::S => 2,
                        Dir::W => 3,
                    }];

                    draw_masked(&mut frame, gx, gy, 2, gpic);
                }
            }

            gx += CELLSIZE;
        }

        gy += CELLSIZE;
    }

    gif.draw_frame(frame, 2)
}

fn guard_pos(input: &[BoardLine]) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| {
                    if matches!(*c, Space::Guard(_, _)) {
                        Some(x)
                    } else {
                        None
                    }
                })
                .map(|x| (x, y))
        })
        .expect("Unable to find the guard")
}

#[derive(Clone, PartialEq)]
enum Space {
    Blocked,
    Empty(u8),
    Guard(Dir, u8),
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

fn input_transform(line: &str) -> BoardLine {
    line.chars()
        .map(|c| match c {
            '.' => Space::Empty(0),
            '#' => Space::Blocked,
            '^' => Space::Guard(Dir::N, 1),
            _ => panic!("Invalid board char {c}"),
        })
        .collect()
}
