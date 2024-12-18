use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
};

use aoc::input::parse_input_vec;

const DIM: usize = 70;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(DIM, 1024, &input));
    println!("Part 2: {}", part2(DIM, &input));

    Ok(())
}

fn part1(dim: usize, count: usize, input: &[Coord]) -> u64 {
    // Create board
    let board = create_board(input, dim, count);

    // Find shortest path
    shortest_path(&board, dim).unwrap() as u64
}

fn part2(dim: usize, input: &[Coord]) -> String {
    // Binary chop the list to find the first time a path can't be made to the target
    let length = input.len();
    let mut half = length / 2;
    let mut rind = length - 1;
    let mut lind = 1;
    let mut fail_point = length;

    while lind <= rind {
        // Create board
        let board = create_board(input, dim, half);

        // Try to find shortest path
        if shortest_path(&board, dim).is_some() {
            // Successful
            lind = half + 1
        } else {
            // No path to the exit
            fail_point = fail_point.min(half - 1);
            rind = half - 1;
        }

        // Find mid point
        half = (rind + lind) / 2;
    }

    format!("{},{}", input[fail_point].0, input[fail_point].1)
}

type Coord = (usize, usize);

fn create_board(input: &[Coord], dim: usize, count: usize) -> Vec<Vec<bool>> {
    // Create board
    let mut board = vec![vec![false; dim + 1]; dim + 1];

    // Corrupt memory
    input.iter().take(count).for_each(|&(x, y)| {
        board[y][x] = true;
    });

    board
}

fn shortest_path(board: &[Vec<bool>], dim: usize) -> Option<usize> {
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
        dist: dist(start),
        steps: 0,
    });

    // Create visited hashmap
    let mut visited = HashMap::new();

    // Process work queue
    while let Some(work) = queue.pop() {
        // Already visited?
        if let Some(len) = visited.get_mut(&work.coord) {
            // Yes - visited in fewer steps?
            if *len <= work.steps {
                // Yes - skip
                continue;
            }

            // No - update fewest steps
            *len = work.steps;
        } else {
            // No - mark as visited
            visited.insert(work.coord, work.steps);
        }

        // Reached end point?
        if work.coord == end {
            // Yes
            continue;
        }

        for next in pos_from(board, work.coord, dim) {
            queue.push(Work {
                coord: next,
                dist: dist(next),
                steps: work.steps + 1,
            });
        }
    }

    visited.get(&end).copied()
}

#[derive(PartialEq, Eq)]
struct Work {
    coord: Coord,
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

fn pos_from(board: &[Vec<bool>], c: Coord, dim: usize) -> impl Iterator<Item = Coord> {
    DIRS.into_iter().filter_map(move |[dx, dy]| {
        match c.0.checked_add_signed(dx) {
            Some(nx) if nx <= dim => match c.1.checked_add_signed(dy) {
                Some(ny) if ny <= dim => {
                    if !board[ny][nx] {
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

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(6, 12, &input), 22);
        assert_eq!(part2(6, &input), "6,1");
    }
}
