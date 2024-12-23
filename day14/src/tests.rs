use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

#[test]
fn test2() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

    let board = Board::new(11, 7, &input);
    assert_eq!(part1(board), 12);
}
