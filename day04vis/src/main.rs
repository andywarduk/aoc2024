use std::error::Error;

use aoc::{gif::Gif, input::parse_input_vec};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(4, input_transform)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

const DIRECTIONS: [(i8, i8); 8] = [
    (1, 0),   // E
    (1, 1),   // SE
    (0, 1),   // S
    (-1, 1),  // SW
    (-1, 0),  // W
    (-1, -1), // NW
    (0, -1),  // N
    (1, -1),  // NE
];

const CELL_SIZE: usize = 7;

fn part1(input: &[InputEnt]) -> Result<(), Box<dyn Error>> {
    let mut hits = vec![vec![0u8; input[0].len()]; input.len()];

    let check_word = |mut x: isize, mut y: isize, dx: isize, dy: isize| -> bool {
        let maxx = input[0].len() as isize;
        let maxy = input.len() as isize;

        // Check for MAS in the given direction
        for i in 1..4 {
            // Change X
            x += dx;

            // Bounds check
            if x < 0 || x >= maxx {
                return false;
            }

            // Change Y
            y += dy;

            // Bounds check
            if y < 0 || y >= maxy {
                return false;
            }

            // Check the board
            if input[y as usize][x as usize] != i {
                return false;
            }
        }

        true
    };

    // Loop each board position
    for (y, r) in input.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            // Got an X?
            if *c == 0 {
                // Search in all directions
                for (dx, dy) in DIRECTIONS {
                    // Check for the word in this direction
                    if check_word(x as isize, y as isize, dx as isize, dy as isize) {
                        // Word found
                        for i in 0..4 {
                            let px = (x as isize + (dx * i) as isize) as usize;
                            let py = (y as isize + (dy * i) as isize) as usize;

                            hits[py][px] += 1;
                        }
                    }
                }
            }
        }
    }

    draw_hits(input, &hits, "vis/day04-1.gif")
}

fn part2(input: &[InputEnt]) -> Result<(), Box<dyn Error>> {
    let mut hits = vec![vec![0u8; input[0].len()]; input.len()];

    // Function to check we have M and S or S and M in the board contents provided
    let check = |a, b| matches!((a, b), (1, 3) | (3, 1));

    // Loop the board skipping the first and last rows and columns
    for (y, r) in input.iter().enumerate().rev().skip(1).rev().skip(1) {
        for (x, c) in r.iter().enumerate().rev().skip(1).rev().skip(1) {
            // Check for A and call the check function with contents of the diagonals
            if *c == 2
                && check(input[y - 1][x - 1], input[y + 1][x + 1])
                && check(input[y - 1][x + 1], input[y + 1][x - 1])
            {
                // Found
                hits[y][x] += 1;
                hits[y - 1][x - 1] += 1;
                hits[y + 1][x - 1] += 1;
                hits[y - 1][x + 1] += 1;
                hits[y + 1][x + 1] += 1;
            }
        }
    }

    draw_hits(input, &hits, "vis/day04-2.gif")
}

const LETTERS: [[[u8; 5]; 5]; 4] = [
    [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
    ],
    [
        [1, 0, 0, 0, 1],
        [1, 1, 0, 1, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ],
    [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ],
    [
        [0, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ],
];

fn draw_hits(input: &[InputEnt], hits: &[Vec<u8>], file: &str) -> Result<(), Box<dyn Error>> {
    let max_hits = hits
        .iter()
        .map(|l| l.iter().cloned().max().unwrap())
        .max()
        .unwrap();

    println!("max hits is {max_hits}");

    let mut palette = Vec::new();

    palette.push([0, 0, 0]);

    // Background colours
    for i in 1..=max_hits {
        let c = 127 + (i * (128 / max_hits));
        palette.push([c, c, c]);
    }

    // No hit text colour
    palette.push([255, 0, 0]);

    let width = (input[0].len() * CELL_SIZE) as u16;
    let height = (input.len() * CELL_SIZE) as u16;

    let mut gif = Gif::new(file, &palette, width, height, 1, 1)?;

    let mut frame_data = vec![vec![0; width as usize]; height as usize];

    for (hy, hl) in hits.iter().enumerate() {
        let gy = hy * CELL_SIZE;

        for (hx, hc) in hl.iter().enumerate() {
            let gx = hx * CELL_SIZE;

            // Fill background
            for y in 0..CELL_SIZE {
                for x in 0..CELL_SIZE {
                    frame_data[gy + y][gx + x] = *hc;
                }
            }

            // Draw letter
            let letter = &LETTERS[input[hy][hx] as usize];
            let text_colour = if *hc == 0 { palette.len() - 1 } else { 0 } as u8;

            for (ly, ll) in letter.iter().enumerate() {
                for (lx, lc) in ll.iter().enumerate() {
                    if *lc == 1 {
                        frame_data[gy + ly + 1][gx + lx + 1] = text_colour;
                    }
                }
            }
        }
    }

    gif.draw_frame(frame_data, 0)?;

    Ok(())
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: String) -> InputEnt {
    // Convert board chars to word letter index
    line.chars()
        .map(|c| match c {
            'X' => 0,
            'M' => 1,
            'A' => 2,
            'S' => 3,
            _ => panic!("Invalid char {c}"),
        })
        .collect()
}
