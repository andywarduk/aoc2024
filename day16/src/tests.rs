use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let graph = build_graph(&input);
    let (best_score, best_routes) = walk(&graph);

    assert_eq!(best_score, 7036);
    assert_eq!(part2(&graph, &best_routes), 45);
}

#[test]
fn test2() {
    let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
    let graph = build_graph(&input);
    let (best_score, best_routes) = walk(&graph);

    assert_eq!(best_score, 11048);
    assert_eq!(part2(&graph, &best_routes), 64);
}
