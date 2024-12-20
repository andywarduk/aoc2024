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

fn input_transform(line: String) -> MapLine {
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
mod tests {
    use std::collections::BTreeMap;

    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE, input_transform).unwrap();

        let path = find_path(&input);

        assert_eq!(path.len(), 84 + 1);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE, input_transform).unwrap();
        let pathmap = find_path(&input);
        let mut cheat_map = cheat_map(find_cheats(&input, &pathmap, 2, 2)).into_iter();

        // There are 14 cheats that save 2 picoseconds.
        // There are 14 cheats that save 4 picoseconds.
        // There are 2 cheats that save 6 picoseconds.
        // There are 4 cheats that save 8 picoseconds.
        // There are 2 cheats that save 10 picoseconds.
        // There are 3 cheats that save 12 picoseconds.
        // There is one cheat that saves 20 picoseconds.
        // There is one cheat that saves 36 picoseconds.
        // There is one cheat that saves 38 picoseconds.
        // There is one cheat that saves 40 picoseconds.
        // There is one cheat that saves 64 picoseconds.

        assert_eq!(cheat_map.next(), Some((2, 14)));
        assert_eq!(cheat_map.next(), Some((4, 14)));
        assert_eq!(cheat_map.next(), Some((6, 2)));
        assert_eq!(cheat_map.next(), Some((8, 4)));
        assert_eq!(cheat_map.next(), Some((10, 2)));
        assert_eq!(cheat_map.next(), Some((12, 3)));
        assert_eq!(cheat_map.next(), Some((20, 1)));
        assert_eq!(cheat_map.next(), Some((36, 1)));
        assert_eq!(cheat_map.next(), Some((38, 1)));
        assert_eq!(cheat_map.next(), Some((40, 1)));
        assert_eq!(cheat_map.next(), Some((64, 1)));
        assert_eq!(cheat_map.next(), None);
    }

    #[test]
    fn test3() {
        let input = parse_test_vec(EXAMPLE, input_transform).unwrap();
        let pathmap = find_path(&input);
        let mut cheat_map = cheat_map(find_cheats(&input, &pathmap, 20, 50)).into_iter();

        // There are 32 cheats that save 50 picoseconds.
        // There are 31 cheats that save 52 picoseconds.
        // There are 29 cheats that save 54 picoseconds.
        // There are 39 cheats that save 56 picoseconds.
        // There are 25 cheats that save 58 picoseconds.
        // There are 23 cheats that save 60 picoseconds.
        // There are 20 cheats that save 62 picoseconds.
        // There are 19 cheats that save 64 picoseconds.
        // There are 12 cheats that save 66 picoseconds.
        // There are 14 cheats that save 68 picoseconds.
        // There are 12 cheats that save 70 picoseconds.
        // There are 22 cheats that save 72 picoseconds.
        // There are 4 cheats that save 74 picoseconds.
        // There are 3 cheats that save 76 picoseconds.

        assert_eq!(cheat_map.next(), Some((50, 32)));
        assert_eq!(cheat_map.next(), Some((52, 31)));
        assert_eq!(cheat_map.next(), Some((54, 29)));
        assert_eq!(cheat_map.next(), Some((56, 39)));
        assert_eq!(cheat_map.next(), Some((58, 25)));
        assert_eq!(cheat_map.next(), Some((60, 23)));
        assert_eq!(cheat_map.next(), Some((62, 20)));
        assert_eq!(cheat_map.next(), Some((64, 19)));
        assert_eq!(cheat_map.next(), Some((66, 12)));
        assert_eq!(cheat_map.next(), Some((68, 14)));
        assert_eq!(cheat_map.next(), Some((70, 12)));
        assert_eq!(cheat_map.next(), Some((72, 22)));
        assert_eq!(cheat_map.next(), Some((74, 4)));
        assert_eq!(cheat_map.next(), Some((76, 3)));
        assert_eq!(cheat_map.next(), None);
    }

    #[test]
    fn test4() {
        let map = vec![vec![Tile::Empty; 7]; 7];

        let mut jumps = cheat_jumps(&map, (3, 3), 3);

        assert_eq!(jumps.next(), Some((3, 0)));
        assert_eq!(jumps.next(), Some((6, 3)));
        assert_eq!(jumps.next(), Some((3, 6)));
        assert_eq!(jumps.next(), Some((0, 3)));
        assert_eq!(jumps.next(), Some((4, 1)));
        assert_eq!(jumps.next(), Some((5, 4)));
        assert_eq!(jumps.next(), Some((2, 5)));
        assert_eq!(jumps.next(), Some((1, 2)));
        assert_eq!(jumps.next(), Some((5, 2)));
        assert_eq!(jumps.next(), Some((4, 5)));
        assert_eq!(jumps.next(), Some((1, 4)));
        assert_eq!(jumps.next(), Some((2, 1)));

        assert_eq!(jumps.next(), None);
    }

    #[test]
    fn test5() {
        let map = vec![vec![Tile::Empty; 5]; 5];

        let mut jumps = cheat_jumps(&map, (2, 2), 3);

        assert_eq!(jumps.next(), Some((3, 0)));
        assert_eq!(jumps.next(), Some((4, 3)));
        assert_eq!(jumps.next(), Some((1, 4)));
        assert_eq!(jumps.next(), Some((0, 1)));
        assert_eq!(jumps.next(), Some((4, 1)));
        assert_eq!(jumps.next(), Some((3, 4)));
        assert_eq!(jumps.next(), Some((0, 3)));
        assert_eq!(jumps.next(), Some((1, 0)));

        assert_eq!(jumps.next(), None);
    }

    fn cheat_map(cheats: impl Iterator<Item = usize>) -> BTreeMap<usize, u8> {
        let mut cheat_map = BTreeMap::new();

        cheats.for_each(|saved| {
            *cheat_map.entry(saved).or_insert(0) += 1u8;
        });

        cheat_map
    }
}
