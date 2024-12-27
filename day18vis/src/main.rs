use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    error::Error,
};

use aoc::{gif::Gif, input::parse_input_vec};
use hsl::HSL;

const DIM: usize = 70;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform)?;

    let last_ok = last_ok(&input);

    draw("vis/day18.gif", &input, last_ok)?;

    Ok(())
}

fn last_ok(input: &[Coord]) -> usize {
    // Binary chop the list to find the last time a path can be made to the target
    let length = input.len();
    let mut half = length / 2;
    let mut rind = length - 1;
    let mut lind = 1;
    let mut last_ok = 0;

    while lind <= rind {
        // Create board
        let board = create_board(input, half);

        // Try to find shortest path
        if shortest_path(&board).is_some() {
            // Successful
            lind = half + 1;
            last_ok = last_ok.max(half);
        } else {
            // No path to the exit
            rind = half - 1;
        }

        // Find mid point
        half = (rind + lind) / 2;
    }

    last_ok
}

const COLOURS: usize = 200;
const FRAME_SKIP: usize = 4;

fn draw(file: &str, input: &[Coord], count: usize) -> Result<(), Box<dyn Error>> {
    // Build palette
    let mut palette = vec![[0, 0, 0], [196, 0, 0], [255, 255, 255]];

    let col_start = palette.len();

    for i in 0..COLOURS {
        let hsl = HSL {
            h: (270.0 / COLOURS as f64) * i as f64,
            s: 1.0,
            l: 0.8,
        };

        let c = hsl.to_rgb();

        palette.push([c.0, c.1, c.2]);
    }

    // Create GIF
    let mut gif = Gif::new(file, &palette, (DIM + 1) as u16, (DIM + 1) as u16, 10, 10)?;

    // Create board
    let mut board = vec![vec![0u8; DIM + 1]; DIM + 1];

    // Function to draw the board
    let draw_board = |frame: &mut Vec<Vec<u8>>, board: &[Vec<u8>]| {
        for (y, l) in board.iter().enumerate() {
            for (x, t) in l.iter().enumerate() {
                frame[y][x] = *t;
            }
        }
    };

    let mut last_path = Vec::new();
    let mut shortest = 0;

    // Animate board
    input
        .iter()
        .take(count)
        .enumerate()
        .try_for_each(|(i, &(x, y))| {
            // Update the board
            board[y][x] = (((i * COLOURS) / count) + col_start) as u8;

            if i % FRAME_SKIP == 0 {
                // Draw the board
                let mut frame = gif.empty_frame();

                draw_board(&mut frame, &board);

                // Get shortest path
                let path = shortest_path(&board).unwrap();

                let delay = if path != last_path {
                    last_path = path.clone();

                    if path.len() != shortest {
                        println!(
                            "Shortest path: {} steps (frame {})",
                            path.len(),
                            (i / FRAME_SKIP) + 1
                        );

                        shortest = path.len();

                        10
                    } else {
                        println!("New path (frame {})", (i / FRAME_SKIP) + 1);

                        5
                    }
                } else {
                    2
                };

                for &(x, y) in path.iter() {
                    frame[y][x] = 1;
                }

                gif.draw_frame(frame, delay)
            } else {
                // Skip this frame
                Ok(())
            }
        })?;

    // Draw last frame
    let mut frame = gif.empty_frame();

    draw_board(&mut frame, &board);

    // Get shortest path
    let path = shortest_path(&board).unwrap();

    for &(x, y) in path.iter() {
        frame[y][x] = 1;
    }

    let (bx, by) = input[count];
    println!("Blocker at {bx}x{by}");

    for i in 0..50 {
        // Draw blocker
        frame[by][bx] = 1 + (i % 2);

        gif.draw_frame(frame.clone(), 10)?;
    }

    Ok(())
}

type Coord = (usize, usize);

fn create_board(input: &[Coord], count: usize) -> Vec<Vec<u8>> {
    // Create board
    let mut board = vec![vec![0u8; DIM + 1]; DIM + 1];

    // Corrupt memory
    input.iter().take(count).for_each(|&(x, y)| {
        board[y][x] = 1;
    });

    board
}

fn shortest_path(board: &[Vec<u8>]) -> Option<Vec<Coord>> {
    // Set start point
    let start = (0, 0);

    // Set end point
    let end = (DIM, DIM);

    // Function to calculate distance from the end point
    let dist = |(x, y): Coord| {
        let dx = end.0 - x;
        let dy = end.1 - y;
        let dist = (((dx * dx) + (dy * dy)) as f64).sqrt();
        (dist * 100.0) as usize
    };

    // initialise work queue
    let mut queue = BinaryHeap::new();

    queue.push(Work {
        coord: start,
        from: start,
        dist: dist(start),
        steps: 0,
    });

    // Create visited hashmap
    let mut visited = HashMap::new();

    // Process work queue
    while let Some(work) = queue.pop() {
        // Already visited?
        if let Some((len, from)) = visited.get_mut(&work.coord) {
            // Yes - visited in fewer steps?
            if *len <= work.steps {
                // Yes - skip
                continue;
            }

            // No - update fewest steps
            *len = work.steps;
            *from = work.from;
        } else {
            // No - mark as visited
            visited.insert(work.coord, (work.steps, work.from));
        }

        // Reached end point?
        if work.coord == end {
            // Yes
            continue;
        }

        for next in pos_from(board, work.coord) {
            queue.push(Work {
                coord: next,
                from: work.coord,
                dist: dist(next),
                steps: work.steps + 1,
            });
        }
    }

    if let Some((_, from)) = visited.get(&end) {
        let mut path = Vec::new();
        path.push(end);

        let mut next = *from;

        loop {
            path.push(next);

            if next == start {
                break;
            }

            (_, next) = *visited.get(&next).unwrap();
        }

        Some(path.into_iter().rev().collect::<Vec<_>>())
    } else {
        None
    }
}

#[derive(PartialEq)]
struct Work {
    coord: Coord,
    from: Coord,
    dist: usize,
    steps: usize,
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Work {}

const DIRS: [[isize; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn pos_from(board: &[Vec<u8>], c: Coord) -> impl Iterator<Item = Coord> {
    DIRS.into_iter().filter_map(move |[dx, dy]| {
        match c.0.checked_add_signed(dx) {
            Some(nx) if nx <= DIM => match c.1.checked_add_signed(dy) {
                Some(ny) if ny <= DIM => {
                    if board[ny][nx] == 0 {
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

fn input_transform(line: &str) -> Coord {
    let mut iter = line.split(",").map(|c| c.parse::<usize>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}
