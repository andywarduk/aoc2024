use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
AAAA
BBCD
BBCC
EEEC
";

const EXAMPLE2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

const EXAMPLE3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const EXAMPLE4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

const EXAMPLE5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let shapes = get_shapes(&input);
    assert_eq!(part1(&shapes), 140);
}

#[test]
fn test2() {
    let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
    let shapes = get_shapes(&input);
    assert_eq!(part1(&shapes), 772);
}

#[test]
fn test3() {
    let input = parse_test_vec(EXAMPLE3, input_transform).unwrap();
    let shapes = get_shapes(&input);
    assert_eq!(part1(&shapes), 1930);
}

#[test]
fn test4() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let shapes = get_shapes(&input);
    assert_eq!(part2(&shapes), 80);
}

#[test]
fn test5() {
    let input = parse_test_vec(EXAMPLE4, input_transform).unwrap();
    let shapes = get_shapes(&input);
    assert_eq!(part2(&shapes), 236);
}

#[test]
fn test6() {
    let input = parse_test_vec(EXAMPLE5, input_transform).unwrap();
    let shapes = get_shapes(&input);
    assert_eq!(part2(&shapes), 368);
}

#[test]
fn test7() {
    let input = parse_test_vec(EXAMPLE3, input_transform).unwrap();
    let shapes = get_shapes(&input);
    assert_eq!(part2(&shapes), 1206);
}
