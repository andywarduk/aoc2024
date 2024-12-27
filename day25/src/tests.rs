use super::*;

const EXAMPLE1: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

#[test]
fn test1() {
    let (locks, keys) = parse_input_str(EXAMPLE1);

    println!("locks: {:?}", locks);
    println!("keys: {:?}", keys);

    assert_eq!(part1(&locks, &keys), 3);
}
