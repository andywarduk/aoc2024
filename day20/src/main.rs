use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let map = parse_input_vec(20, input_transform)?;

    // Get map of coord to path index
    let pathmap = find_path(&map);

    // Run parts
    println!("Part 1: {}", part1(&map, &pathmap));
    println!("Part 2: {}", part2(&map, &pathmap));

    Ok(())
}

fn part1(map: &[MapLine], pathmap: &FxHashMap<Coord, usize>) -> u64 {
    // Return number of cheats of length 2 that save at >= 100 picoseconds
    find_cheats(map, pathmap, 2, 100).count() as u64
}

fn part2(map: &[MapLine], pathmap: &FxHashMap<Coord, usize>) -> u64 {
    // Return number of cheats of length 20 that save at >= 100 picoseconds
    find_cheats(map, pathmap, 20, 100).count() as u64
}

fn find_path(map: &[Vec<Tile>]) -> FxHashMap<Coord, usize> {
    // Find start and end positions
    let start = find_tile(map, Tile::Start);
    let end = find_tile(map, Tile::End);

    // Initialsise coord -> path index map
    let mut pathmap = FxHashMap::default();

    // Initial position
    let mut pos = start;

    // Saved positions
    let mut last_pos = pos;

    // Current path index
    let mut idx = 0;

    // Insert initial position
    pathmap.insert(pos, idx);

    // Loop while not at the end
    while pos != end {
        // Get next position
        let next = next_pos(map, pos, last_pos);

        // Insert in to the map
        idx += 1;
        pathmap.insert(next, idx);

        // Move to next
        last_pos = pos;
        pos = next;
    }

    pathmap
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn next_pos(map: &[Vec<Tile>], pos: Coord, last_pos: Coord) -> Coord {
    // Find next adjacent position that is not a wall and is not the last position
    DIRS.iter()
        .map(|(dx, dy)| (pos.0 as isize + dx, pos.1 as isize + dy))
        .find_map(|(x, y)| {
            let x = x as usize;
            let y = y as usize;
            let next = (x, y);

            if next != last_pos && map[y][x] != Tile::Wall {
                Some(next)
            } else {
                None
            }
        })
        .unwrap()
}

fn find_tile(map: &[Vec<Tile>], tile: Tile) -> Coord {
    // Find first tile in map of a given type
    map.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, t)| if *t == tile { Some((x, y)) } else { None })
        })
        .unwrap()
}

fn find_cheats(
    map: &[Vec<Tile>],
    pathmap: &FxHashMap<Coord, usize>,
    duration: usize,
    cutoff: usize,
) -> impl Iterator<Item = usize> {
    // Iterate the path map
    pathmap.iter().flat_map(move |(&pos, &idx)| {
        // Iterate the duration range
        (2..=duration).flat_map(move |duration| {
            // Iterate the valid jump positions for the duration
            cheat_jumps(map, pos, duration).filter_map(move |cheat_pos| {
                // Is this jumped to position on the path?
                if let Some(cheat_idx) = pathmap.get(&cheat_pos) {
                    // Yes - check the position on the path is later than the current position
                    if *cheat_idx > idx && *cheat_idx > duration {
                        // It is - calculate the saved picoseconds
                        let saved = cheat_idx - idx - duration;

                        // Check against cutoff
                        if saved >= cutoff {
                            return Some(saved);
                        }
                    }
                }

                // Jumped to position is not valid
                None
            })
        })
    })
}

fn cheat_jumps(map: &[Vec<Tile>], pos: Coord, duration: usize) -> impl Iterator<Item = Coord> {
    let x = pos.0 as isize;
    let y = pos.1 as isize;

    // Generate the valid jump positions for the duration
    //
    // eg duration = 3:
    //
    //    1
    //   4.1    1 = ne
    //  4...1   2 = se
    // 4..P..2  3 = sw
    //  3...2   4 = nw
    //   3.2    P = position
    //    3

    // Coordinates for each direction given movement a and b
    let ne = move |a: isize, b: isize| -> (isize, isize) { (x + a, y - b) };
    let se = move |a: isize, b: isize| -> (isize, isize) { (x + b, y + a) };
    let sw = move |a: isize, b: isize| -> (isize, isize) { (x - a, y + b) };
    let nw = move |a: isize, b: isize| -> (isize, isize) { (x - b, y - a) };

    // Iterate the duration range and generate the jump position for each direction
    (0..duration)
        .map(move |i| (i as isize, (duration - i) as isize))
        .flat_map(move |(a, b)| [ne(a, b), se(a, b), sw(a, b), nw(a, b)])
        .filter_map(|(x, y)| {
            // Check lower bound
            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;

                // Check upper bound and not a wall
                if y < map.len() && x < map[0].len() && map[y][x] != Tile::Wall {
                    return Some((x, y));
                }
            }

            None
        })
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

// Input parsing

type MapLine = Vec<Tile>;

fn input_transform(line: &str) -> MapLine {
    // Convert chars to tiles
    line.chars()
        .map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => panic!("Invalid tile"),
        })
        .collect()
}

#[cfg(test)]
mod tests;
