use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(20, input_transform)?;

    let path = find_path(&input);

    // Run parts
    println!("Part 1: {}", part1(&input, &path));
    println!("Part 2: {}", part2(&input, &path));

    Ok(())
}

fn part1(input: &[InputEnt], path: &[(usize, usize)]) -> u64 {
    let cheats = find_cheats(input, path, 2, 100);

    cheats.len() as u64
}

fn part2(input: &[InputEnt], path: &[(usize, usize)]) -> u64 {
    let cheats = find_cheats(input, path, 20, 100);

    cheats.len() as u64
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

fn find_cheats(
    map: &[Vec<Tile>],
    path: &[Coord],
    duration: usize,
    cutoff: usize,
) -> Vec<(Coord, usize)> {
    let mut cheats = vec![];

    let hashmap = path
        .iter()
        .enumerate()
        .map(|(i, pos)| (*pos, i))
        .collect::<FxHashMap<Coord, usize>>();

    (0..path.len()).for_each(|i| {
        let (px, py) = path[i];

        for duration in 2..=duration {
            for cheat_pos in cheat_jumps(map, (px, py), duration) {
                if let Some(cheat_idx) = hashmap.get(&cheat_pos) {
                    if *cheat_idx > i && *cheat_idx > duration {
                        let saved = cheat_idx - i - duration;

                        if saved >= cutoff {
                            cheats.push((cheat_pos, saved));
                        }
                    }
                }
            }
        }
    });

    cheats
}

fn cheat_jumps(map: &[Vec<Tile>], pos: Coord, duration: usize) -> impl Iterator<Item = Coord> {
    let (x, y) = pos;

    // eg duration = 3
    //    X
    //   X.X
    //  X...X
    // X..P..X
    //  X...X
    //   X.X
    //    X

    let ne = move |i: usize| -> (isize, isize) { ((i as isize), -((duration - i) as isize)) };
    let se = move |i: usize| -> (isize, isize) { ((duration - i) as isize, i as isize) };
    let sw = move |i: usize| -> (isize, isize) { (-(i as isize), (duration - i) as isize) };
    let nw = move |i: usize| -> (isize, isize) { (-((duration - i) as isize), -(i as isize)) };

    (0..duration)
        .map(ne)
        .chain((0..duration).map(se))
        .chain((0..duration).map(sw))
        .chain((0..duration).map(nw))
        .filter_map(move |(dx, dy)| {
            let x = x as isize + dx;
            let y = y as isize + dy;

            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;

                if y < map.len() && x < map[0].len() && map[y][x] != Tile::Wall {
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
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

        let cheats = find_cheats(&input, &path, 2, 0);

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

        let path = find_path(&input);

        let cheats = find_cheats(&input, &path, 20, 50);

        let mut cheat_map = BTreeMap::new();

        cheats.iter().for_each(|(_, saved)| {
            *cheat_map.entry(*saved).or_insert(0) += 1u8;
        });

        let mut map_iter = cheat_map.into_iter();

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

        assert_eq!(map_iter.next(), Some((50, 32)));
        assert_eq!(map_iter.next(), Some((52, 31)));
        assert_eq!(map_iter.next(), Some((54, 29)));
        assert_eq!(map_iter.next(), Some((56, 39)));
        assert_eq!(map_iter.next(), Some((58, 25)));
        assert_eq!(map_iter.next(), Some((60, 23)));
        assert_eq!(map_iter.next(), Some((62, 20)));
        assert_eq!(map_iter.next(), Some((64, 19)));
        assert_eq!(map_iter.next(), Some((66, 12)));
        assert_eq!(map_iter.next(), Some((68, 14)));
        assert_eq!(map_iter.next(), Some((70, 12)));
        assert_eq!(map_iter.next(), Some((72, 22)));
        assert_eq!(map_iter.next(), Some((74, 4)));
        assert_eq!(map_iter.next(), Some((76, 3)));
        assert_eq!(map_iter.next(), None);
    }

    #[test]
    fn test4() {
        let map = vec![vec![Tile::Empty; 7]; 7];

        let mut jumps = cheat_jumps(&map, (3, 3), 3);

        // NE
        assert_eq!(jumps.next(), Some((3, 0)));
        assert_eq!(jumps.next(), Some((4, 1)));
        assert_eq!(jumps.next(), Some((5, 2)));

        // SE
        assert_eq!(jumps.next(), Some((6, 3)));
        assert_eq!(jumps.next(), Some((5, 4)));
        assert_eq!(jumps.next(), Some((4, 5)));

        // SW
        assert_eq!(jumps.next(), Some((3, 6)));
        assert_eq!(jumps.next(), Some((2, 5)));
        assert_eq!(jumps.next(), Some((1, 4)));

        // NW
        assert_eq!(jumps.next(), Some((0, 3)));
        assert_eq!(jumps.next(), Some((1, 2)));
        assert_eq!(jumps.next(), Some((2, 1)));

        assert_eq!(jumps.next(), None);
    }

    #[test]
    fn test5() {
        let map = vec![vec![Tile::Empty; 5]; 5];

        let mut jumps = cheat_jumps(&map, (2, 2), 3);

        // NE
        assert_eq!(jumps.next(), Some((3, 0)));
        assert_eq!(jumps.next(), Some((4, 1)));

        // SE
        assert_eq!(jumps.next(), Some((4, 3)));
        assert_eq!(jumps.next(), Some((3, 4)));

        // SW
        assert_eq!(jumps.next(), Some((1, 4)));
        assert_eq!(jumps.next(), Some((0, 3)));

        // NW
        assert_eq!(jumps.next(), Some((0, 1)));
        assert_eq!(jumps.next(), Some((1, 0)));

        assert_eq!(jumps.next(), None);
    }
}
