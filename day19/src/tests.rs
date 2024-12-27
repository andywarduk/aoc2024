use super::*;

const EXAMPLE1: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

#[test]
fn test1() {
    let (available, designs) = parse_input_str(EXAMPLE1);

    let composable = build_composable(&available, &designs);

    assert_eq!(part1(&composable), 6);
    assert_eq!(part2(&composable), 16);
}
