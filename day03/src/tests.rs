use super::*;

const EXAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE1), 161);
}

#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE2), 48);
}
