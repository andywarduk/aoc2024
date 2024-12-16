use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(16, input_transform)?;
    let graph = build_graph(&input);
    let (best_score, best_routes) = walk(&graph);

    // Run parts
    println!("Part 1: {}", best_score);
    println!("Part 2: {}", part2(&graph, &best_routes));

    Ok(())
}

fn part2(graph: &Graph, best_routes: &[Vec<usize>]) -> u64 {
    let coords = best_routes
        .iter()
        .flat_map(|r| r.iter().flat_map(|e| graph.edges[*e].path.iter().copied()))
        .collect::<HashSet<Coord>>();

    coords.len() as u64
}

fn walk(graph: &Graph) -> (u64, Vec<Vec<usize>>) {
    let mut best_score = u64::MAX;
    let mut best_routes = Vec::new();

    let mut scores = HashMap::new();

    let mut workq = VecDeque::new();

    workq.push_back(Work {
        node: graph.start,
        dir: Dir::E,
        score: 0,
        route: Vec::new(),
    });

    while let Some(work) = workq.pop_front() {
        if work.node == graph.end {
            if work.score < best_score {
                best_score = work.score;
                best_routes = vec![work.route];
            } else if work.score == best_score {
                best_routes.push(work.route);
            }
            continue;
        }

        if let Some(score) = scores.get_mut(&(work.node, work.dir)) {
            if *score < work.score {
                continue;
            }
            *score = work.score;
        } else {
            scores.insert((work.node, work.dir), work.score);
        }

        for en in graph.nodes[work.node].edges.iter() {
            let edge = &graph.edges[*en];

            if work.dir.opposite() == edge.indir {
                continue;
            }

            let mut score = work.score + edge.score;

            if work.dir != edge.indir {
                score += 1000;
            }

            if score > best_score {
                continue;
            }

            let mut new_route = work.route.clone();

            new_route.push(*en);

            workq.push_back(Work {
                node: edge.tonode,
                dir: edge.outdir,
                score,
                route: new_route,
            });
        }
    }

    (best_score, best_routes)
}

struct Work {
    node: usize,
    dir: Dir,
    score: u64,
    route: Vec<usize>,
}

struct Node {
    pos: Coord,
    edges: Vec<usize>,
}

struct Edge {
    tonode: usize,
    indir: Dir,
    outdir: Dir,
    score: u64,
    path: Vec<Coord>,
}

struct Graph {
    start: usize,
    end: usize,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn build_graph(input: &[InputEnt]) -> Graph {
    // Find start
    let spos = input
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == MapTile::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    // Find end
    let epos = input
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == MapTile::End {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    // Find nodes
    let mut nodes = Vec::new();

    for (y, l) in input.iter().enumerate() {
        for (x, t) in l.iter().enumerate() {
            if *t == MapTile::Wall {
                continue;
            }

            let pos = (x, y);
            let dirs = dirs(input, pos, None);

            if dirs.len() > 2 || pos == spos || pos == epos {
                nodes.push(Node {
                    pos,
                    edges: Vec::new(),
                });
            }
        }
    }

    let node_map = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.pos, i))
        .collect::<HashMap<_, _>>();

    // Build edges
    let mut edges = Vec::new();

    for n in nodes.iter_mut() {
        for (dir, mut next) in dirs(input, n.pos, None) {
            let mut cdir = dir;
            let mut path = Vec::new();
            let mut score = 1;

            path.push(n.pos);

            loop {
                path.push(next);

                let dirs = dirs(input, next, Some(cdir.opposite()));

                if dirs.is_empty() {
                    break;
                }

                if let Some(n2) = node_map.get(&next) {
                    n.edges.push(edges.len());

                    edges.push(Edge {
                        tonode: *n2,
                        indir: dir,
                        outdir: cdir,
                        score,
                        path,
                    });

                    break;
                }

                if dirs[0].0 != cdir {
                    score += 1000;
                    cdir = dirs[0].0;
                }

                next = dirs[0].1;
                score += 1;
            }
        }
    }

    let start = *node_map.get(&spos).unwrap();
    let end = *node_map.get(&epos).unwrap();

    Graph {
        start,
        end,
        nodes,
        edges,
    }
}

type Coord = (usize, usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

const DIRS: [(Dir, [isize; 2]); 4] = [
    (Dir::N, [0, -1]),
    (Dir::E, [1, 0]),
    (Dir::S, [0, 1]),
    (Dir::W, [-1, 0]),
];

fn dirs(input: &[InputEnt], c: Coord, skip_dir: Option<Dir>) -> Vec<(Dir, Coord)> {
    DIRS.iter()
        .filter_map(move |&(mdir, [dx, dy])| match c.0.checked_add_signed(dx) {
            Some(nx) if nx < input[0].len() => match c.1.checked_add_signed(dy) {
                Some(ny) if ny < input.len() => {
                    if input[ny][nx] != MapTile::Wall {
                        if let Some(skip_dir) = skip_dir {
                            if mdir == skip_dir {
                                None
                            } else {
                                Some((mdir, (nx, ny)))
                            }
                        } else {
                            Some((mdir, (nx, ny)))
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        })
        .collect()
}

// Input parsing

#[derive(PartialEq)]
enum MapTile {
    Empty,
    Wall,
    Start,
    End,
}

type InputEnt = Vec<MapTile>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => MapTile::Empty,
            '#' => MapTile::Wall,
            'S' => MapTile::Start,
            'E' => MapTile::End,
            _ => panic!("Bad map tile"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
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
}
