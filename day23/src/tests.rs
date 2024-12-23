use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let graph = build_graph(input);

    assert_eq!(part1(&graph), 7);
    assert_eq!(part2(&graph), "co,de,ka,ta");
}
