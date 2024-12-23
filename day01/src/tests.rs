use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let (v1, v2) = split_input(&input);

    assert_eq!(part1(&v1, &v2), 11);
    assert_eq!(part2(&v1, &v2), 31);
}
