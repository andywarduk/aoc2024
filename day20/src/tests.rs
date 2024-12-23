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
