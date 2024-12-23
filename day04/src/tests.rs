use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    assert_eq!(part1(&input), 18);
    assert_eq!(part2(&input), 9);
}
