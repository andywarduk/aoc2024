use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[test]
fn test1() {
    let mut input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

    let (p1, p2) = run_parts(&mut input);

    assert_eq!(p1, 41);
    assert_eq!(p2, 6);
}
