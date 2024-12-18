use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
};

use aoc::{gif::Gif, input::parse_input_vec};
use hsl::HSL;

const DIM: usize = 70;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform)?;

    let last_ok = last_ok(DIM, &input);

    draw("vis/day18.gif", &input, DIM, last_ok)?;

    Ok(())
}

fn last_ok(dim: usize, input: &[Coord]) -> usize {
    // Binary chop the list to find the last time a path can be made to the target
    let length = input.len();
    let mut half = length / 2;
    let mut rind = length - 1;
    let mut lind = 1;
    let mut last_ok = 0;

    while lind <= rind {
        // Create board
        let board = create_board(input, dim, half);

        // Try to find shortest path
        if shortest_path(&board, dim).is_some() {
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

fn draw(file: &str, input: &[Coord], dim: usize, count: usize) -> Result<(), Box<dyn Error>> {
    let mut palette = vec![[0, 0, 0], [196, 0, 0]];

    for i in 0..COLOURS {
        let hsl = HSL {
            h: (270.0 / COLOURS as f64) * i as f64,
            s: 1.0,
            l: 0.8,
        };

        let c = hsl.to_rgb();

        palette.push([c.0, c.1, c.2]);
    }

    let mut gif = Gif::new(file, &palette, (dim + 1) as u16, (dim + 1) as u16, 10, 10)?;

    // Create board
    let mut board = vec![vec![0u8; dim + 1]; dim + 1];

    let draw_board = |frame: &mut Vec<Vec<u8>>, board: &[Vec<u8>]| {
        for (y, l) in board.iter().enumerate() {
            for (x, t) in l.iter().enumerate() {
                frame[y][x] = *t;
            }
        }
    };

    // Animate board
    input
        .iter()
        .take(count)
        .enumerate()
        .try_for_each(|(i, &(x, y))| {
            board[y][x] = ((i * COLOURS) / count) as u8 + 2;

            let mut frame = gif.empty_frame();
            draw_board(&mut frame, &board);

            if i % 8 == 0 {
                gif.draw_frame(frame, 2)
            } else {
                Ok(())
            }
        })?;

    // Get shortest path
    let path = shortest_path(&board, dim).unwrap();

    // Animate path
    for i in 0..=path.len() {
        let mut frame = gif.empty_frame();

        // Draw board
        draw_board(&mut frame, &board);

        // Draw path
        for &(x, y) in path.iter().take(i) {
            frame[y][x] = 1;
        }

        gif.draw_frame(frame, 2)?;
    }

    gif.delay(500)?;

    Ok(())
}

type Coord = (usize, usize);

fn create_board(input: &[Coord], dim: usize, count: usize) -> Vec<Vec<u8>> {
    // Create board
    let mut board = vec![vec![0u8; dim + 1]; dim + 1];

    // Corrupt memory
    input.iter().take(count).for_each(|&(x, y)| {
        board[y][x] = 1;
    });

    board
}

fn shortest_path(board: &[Vec<u8>], dim: usize) -> Option<Vec<Coord>> {
    // Set start point
    let start = (0, 0);

    // Set end point
    let end = (dim, dim);

    // Function to calculate manhattan distance from the end point
    let dist = |(x, y)| (end.0 - x) + (end.1 - y);

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

        for next in pos_from(board, work.coord, dim) {
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

#[derive(PartialEq, Eq)]
struct Work {
    coord: Coord,
    from: Coord,
    dist: usize,
    steps: usize,
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const DIRS: [[isize; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn pos_from(board: &[Vec<u8>], c: Coord, dim: usize) -> impl Iterator<Item = Coord> {
    DIRS.into_iter().filter_map(move |[dx, dy]| {
        match c.0.checked_add_signed(dx) {
            Some(nx) if nx <= dim => match c.1.checked_add_signed(dy) {
                Some(ny) if ny <= dim => {
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

fn input_transform(line: String) -> Coord {
    let mut iter = line.split(",").map(|c| c.parse::<usize>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}
