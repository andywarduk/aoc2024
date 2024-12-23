use super::*;

const EXAMPLE1: &str = "0 1 10 99 999";
const EXAMPLE2: &str = "125 17";

#[test]
fn test1() {
    let input = input_transform(EXAMPLE1.to_string());
    assert_eq!(count(&input, 1), 7);
}

#[test]
fn test2() {
    let input = input_transform(EXAMPLE2.to_string());
    assert_eq!(count(&input, 6), 22);
}

#[test]
fn test3() {
    let input = input_transform(EXAMPLE2.to_string());
    assert_eq!(part1(&input), 55312);
}
