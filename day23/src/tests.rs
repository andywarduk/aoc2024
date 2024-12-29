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

const CONNECTED_3: &str = "\
aq,cg,yn
aq,vc,wq
co,de,ka
co,de,ta
co,ka,ta
de,ka,ta
kh,qp,ub
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn
ub,vc,wq
";

const CONNECTED_3T: &str = "\
co,de,ta
co,ka,ta
de,ka,ta
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn
";

#[test]
fn test1() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let graph = build_graph(input);
    let connected = CONNECTED_3.lines().collect::<Vec<_>>();
    let mut found = Vec::new();

    graph.walk(&mut |set| {
        if set.len() == 3 {
            found.push(set.join(","));
            false
        } else {
            true
        }
    });

    found.sort();

    assert_eq!(found, connected);
}

#[test]
fn test2() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let graph = build_graph(input);
    let connected = CONNECTED_3T.lines().collect::<Vec<_>>();
    let mut found = Vec::new();

    graph.walk(&mut |set| {
        if set.len() == 3 {
            if set.iter().any(|n| n.starts_with('t')) {
                found.push(set.join(","));
            }

            false
        } else {
            true
        }
    });

    found.sort();

    assert_eq!(found, connected);
}

#[test]
fn test3() {
    let mut graph = Graph::default();

    graph.add_edge("1", "2");
    graph.add_edge("1", "5");
    graph.add_edge("2", "3");
    graph.add_edge("2", "5");
    graph.add_edge("3", "4");
    graph.add_edge("4", "5");
    graph.add_edge("4", "6");

    assert_eq!(graph.max_cliques(), vec![vec!["1", "2", "5"]]);
}

#[test]
fn test4() {
    let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
    let graph = build_graph(input);

    assert_eq!(part1(&graph), 7);
    assert_eq!(part2(&graph), "co,de,ka,ta");
}
