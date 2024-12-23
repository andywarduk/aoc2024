use super::*;

const EXAMPLE1: &str = "2333133121414131402";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE1), 1928);
    assert_eq!(part2(EXAMPLE1), 2858);
}
