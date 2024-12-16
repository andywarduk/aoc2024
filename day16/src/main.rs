use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(16, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    // Find start
    let spos = input
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == MapTile::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    // Find end
    let epos = input
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == MapTile::End {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut workq = VecDeque::new();

    workq.push_back(Work1 {
        pos: spos,
        dir: Dir::E,
        score: 0,
        route: Vec::new(),
    });

    let mut best_score = u64::MAX;

    let mut scores = HashMap::new();

    while let Some(work) = workq.pop_front() {
        if work.pos == epos {
            println!("{}: {:?}", work.score, work.route);
            if work.score < best_score {
                best_score = work.score;
            }
            continue;
        }

        let new_dirs = dirs(input, work.pos);

        let mut add_route = false;

        if new_dirs.len() > 2 {
            // > 2 choice of direction
            if let Some(score) = scores.get(&(work.pos, work.dir)) {
                if *score < work.score {
                    continue;
                }
            } else {
                scores.insert((work.pos, work.dir), work.score);
            }

            add_route = true;
        }

        for (new_dir, new_pos) in new_dirs {
            if work.dir.opposite() == new_dir {
                continue;
            }

            let mut score = work.score + 1;

            if work.dir != new_dir {
                score += 1000;
            }

            if score > best_score {
                continue;
            }

            let mut new_route = work.route.clone();

            if add_route {
                new_route.push((work.pos, new_dir));
            }

            workq.push_back(Work1 {
                pos: new_pos,
                dir: new_dir,
                score,
                route: new_route,
            })
        }
    }

    best_score
}

struct Work1 {
    pos: Coord,
    dir: Dir,
    score: u64,
    route: Vec<(Coord, Dir)>,
}

fn part2(input: &[InputEnt]) -> u64 {
    0 // TODO
}

type Coord = (usize, usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

const DIRS: [(Dir, [isize; 2]); 4] = [
    (Dir::N, [0, -1]),
    (Dir::E, [1, 0]),
    (Dir::S, [0, 1]),
    (Dir::W, [-1, 0]),
];

fn dirs(input: &[InputEnt], c: Coord) -> Vec<(Dir, Coord)> {
    DIRS.iter()
        .filter_map(move |&(mdir, [dx, dy])| match c.0.checked_add_signed(dx) {
            Some(nx) if nx < input[0].len() => match c.1.checked_add_signed(dy) {
                Some(ny) if ny < input.len() => {
                    if input[ny][nx] != MapTile::Wall {
                        Some((mdir, (nx, ny)))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        })
        .collect()
}

// Input parsing

#[derive(PartialEq)]
enum MapTile {
    Empty,
    Wall,
    Start,
    End,
}

type InputEnt = Vec<MapTile>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => MapTile::Empty,
            '#' => MapTile::Wall,
            'S' => MapTile::Start,
            'E' => MapTile::End,
            _ => panic!("Bad map tile"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 7036);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        assert_eq!(part1(&input), 11048);
    }
}
