use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    assert_eq!(part1(&input), 36);
    assert_eq!(part2(&input), 81);
}
