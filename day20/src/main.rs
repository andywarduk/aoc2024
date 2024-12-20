use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(20, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    let path = find_path(input);

    let cheats = find_cheats(input, &path);

    cheats.iter().filter(|(_, saved)| *saved >= 100).count() as u64
}

fn part2(input: &[InputEnt]) -> u64 {
    0 // TODO
}

fn find_path(map: &[Vec<Tile>]) -> Vec<Coord> {
    let start = find_tile(map, Tile::Start);
    let end = find_tile(map, Tile::End);

    let mut path = vec![];

    let mut pos = start;
    path.push(pos);

    while pos != end {
        let last_pos = if path.len() <= 1 {
            None
        } else {
            Some(path[path.len() - 2])
        };

        pos = next_pos(map, pos, last_pos);

        path.push(pos);
    }

    path
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn next_pos(map: &[Vec<Tile>], pos: Coord, last_pos: Option<Coord>) -> Coord {
    DIRS.iter()
        .map(|(dx, dy)| (pos.0 as isize + dx, pos.1 as isize + dy))
        .find_map(|(x, y)| {
            let x = x as usize;
            let y = y as usize;
            let next = (x, y);

            if map[y][x] != Tile::Wall {
                if let Some(last) = last_pos {
                    if next != last { Some(next) } else { None }
                } else {
                    Some(next)
                }
            } else {
                None
            }
        })
        .unwrap()
}

fn find_tile(map: &[Vec<Tile>], tile: Tile) -> Coord {
    map.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, t)| if *t == tile { Some((x, y)) } else { None })
        })
        .unwrap()
}

fn find_cheats(map: &[Vec<Tile>], path: &[Coord]) -> Vec<(Coord, usize)> {
    let mut cheats = vec![];

    let hashmap = path
        .iter()
        .enumerate()
        .map(|(i, pos)| (*pos, i))
        .collect::<FxHashMap<Coord, usize>>();

    (0..path.len()).for_each(|i| {
        let (px, py) = path[i];

        for (dx, dy) in DIRS.iter() {
            let x = px as isize + (dx * 2);
            let y = py as isize + (dy * 2);

            if x < 0 || y < 0 || y >= map.len() as isize || x >= map[0].len() as isize {
                continue;
            }

            let cheat_pos = (x as usize, y as usize);

            if let Some(cheat_idx) = hashmap.get(&cheat_pos) {
                if *cheat_idx > i && *cheat_idx > 2 {
                    let saved = cheat_idx - i - 2;

                    if saved > 0 {
                        cheats.push((cheat_pos, saved));
                    }
                }
            }
        }
    });

    cheats
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

type InputEnt = Vec<Tile>;

fn input_transform(line: String) -> InputEnt {
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

    const EXAMPLE1: &str = "\
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
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        let path = find_path(&input);

        assert_eq!(path.len(), 84 + 1);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        let path = find_path(&input);

        let cheats = find_cheats(&input, &path);

        let mut cheat_map = BTreeMap::new();

        cheats.iter().for_each(|(_, saved)| {
            *cheat_map.entry(*saved).or_insert(0) += 1u8;
        });

        let mut map_iter = cheat_map.into_iter();

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

        assert_eq!(map_iter.next(), Some((2, 14)));
        assert_eq!(map_iter.next(), Some((4, 14)));
        assert_eq!(map_iter.next(), Some((6, 2)));
        assert_eq!(map_iter.next(), Some((8, 4)));
        assert_eq!(map_iter.next(), Some((10, 2)));
        assert_eq!(map_iter.next(), Some((12, 3)));
        assert_eq!(map_iter.next(), Some((20, 1)));
        assert_eq!(map_iter.next(), Some((36, 1)));
        assert_eq!(map_iter.next(), Some((38, 1)));
        assert_eq!(map_iter.next(), Some((40, 1)));
        assert_eq!(map_iter.next(), Some((64, 1)));
        assert_eq!(map_iter.next(), None);
    }

    #[test]
    fn test3() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 0 /* TODO */);
        assert_eq!(part2(&input), 0 /* TODO */);
    }
}
