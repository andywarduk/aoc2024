use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    error::Error,
};

use aoc::{gif::Gif, input::parse_input_vec};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(16, input_transform)?;
    let graph = build_graph(&input);
    walk(&input, &graph, "vis/day16.gif")?;

    Ok(())
}

#[derive(PartialEq, Eq)]
struct Work {
    node: usize,
    dir: Dir,
    score: u64,
    dist: usize,
    route: Vec<usize>,
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| other.dist.cmp(&self.dist))
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const SCALE: usize = 4;

fn walk(input: &[InputEnt], graph: &Graph, file: &str) -> Result<(), Box<dyn Error>> {
    // Build palette
    let mut palette = vec![[0, 0, 0], [128, 128, 255], [80, 80, 0], [255, 0, 0], [
        0, 255, 0,
    ]];

    for i in 0..25 {
        let c = 255 - (i * 8);
        palette.push([c, c, c]);
    }

    // Create GIF
    let mut gif = Gif::new(
        file,
        &palette,
        graph.maxx as u16,
        graph.maxy as u16,
        SCALE as u16,
        SCALE as u16,
    )?;

    let mut frameno = 0;

    let mut best_score = u64::MAX;
    let mut best_routes = Vec::new();

    let mut scores = HashMap::new();

    let mut workq = BinaryHeap::new();

    // Add start point to work queue
    workq.push(Work {
        node: graph.start,
        dir: Dir::E,
        score: 0,
        dist: graph.nodes[graph.start].dist,
        route: Vec::new(),
    });

    // Process work queue
    while let Some(work) = workq.pop() {
        if work.node == graph.end {
            // At the end node - compare best score
            match work.score.cmp(&best_score) {
                Ordering::Less => {
                    // New best score
                    best_score = work.score;
                    best_routes = vec![work.route];
                }
                Ordering::Equal => {
                    // Equal best score
                    best_routes.push(work.route);
                }
                Ordering::Greater => (),
            }

            continue;
        }

        // Visited from this direction before?
        if let Some(score) = scores.get_mut(&(work.node, work.dir)) {
            // Yes - is the score worse?
            if *score < work.score {
                // Yes - ignore
                continue;
            }

            // No - new best score for this node from this direction
            *score = work.score;
        } else {
            // First time visited in this direction
            scores.insert((work.node, work.dir), work.score);
        }

        // Iterate node edges
        for en in graph.nodes[work.node].edges.iter() {
            let edge = &graph.edges[*en];

            // Don't double back
            if work.dir.opposite() == edge.indir {
                continue;
            }

            // Calculate new score
            let mut score = work.score + edge.score;

            if work.dir != edge.indir {
                // Turn needed to enter the edge
                score += 1000;
            }

            // Check current score is not more than the best score
            if score > best_score {
                continue;
            }

            // Build new route
            let mut new_route = work.route.clone();

            new_route.push(*en);

            // Add work queue element
            workq.push(Work {
                node: edge.tonode,
                dir: edge.outdir,
                score,
                dist: graph.nodes[work.node].dist,
                route: new_route,
            });
        }

        frameno += 1;
        if frameno % 12 == 0 {
            draw_workq(&mut gif, input, graph, &workq)?;
        }
    }

    draw_best(&mut gif, input, graph, &best_routes)?;

    gif.delay(1000)?;

    Ok(())
}

fn draw_workq(
    gif: &mut Gif,
    input: &[Vec<MapTile>],
    graph: &Graph,
    workq: &BinaryHeap<Work>,
) -> Result<(), Box<dyn Error>> {
    let mut frame = gif.empty_frame();

    // Draw map
    draw_map(&mut frame, input, graph);

    let draw = workq.len().min(25);

    for (i, work) in workq.iter().enumerate().take(draw).rev() {
        for en in &work.route {
            let edge = &graph.edges[*en];

            for &(x, y) in &edge.path {
                frame[y][x] = (i + 5) as u8;
            }
        }
    }

    draw_startend(&mut frame, graph);

    gif.draw_frame(frame, 2)?;

    Ok(())
}

fn draw_best(
    gif: &mut Gif,
    input: &[Vec<MapTile>],
    graph: &Graph,
    best: &Vec<Vec<usize>>,
) -> Result<(), Box<dyn Error>> {
    let mut counts: HashMap<(usize, usize), usize> = HashMap::new();

    // Get visit counts for each location
    for r in best {
        for edge in r {
            for &c in graph.edges[*edge].path.iter().skip(1) {
                counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    let max_count = counts.values().copied().max().unwrap();
    let col_step = 12 / max_count;

    // Get empty frame
    let mut frame = gif.empty_frame();

    // Draw map
    draw_map(&mut frame, input, graph);

    // Draw best paths
    for ((x, y), c) in counts {
        frame[y][x] = (((max_count - c) * col_step) + 5) as u8;
    }

    draw_startend(&mut frame, graph);

    // Output frame
    gif.draw_frame(frame, 2)?;

    Ok(())
}

fn draw_map(frame: &mut [Vec<u8>], input: &[InputEnt], graph: &Graph) {
    // Draw walls
    for (y, l) in input.iter().enumerate() {
        for (x, t) in l.iter().enumerate() {
            if *t == MapTile::Wall {
                frame[y][x] = 1;
            }
        }
    }

    // Draw nodes
    for n in &graph.nodes {
        frame[n.pos.1][n.pos.0] = 2;
    }
}

fn draw_startend(frame: &mut [Vec<u8>], graph: &Graph) {
    let s = &graph.nodes[graph.start];
    frame[s.pos.1][s.pos.0] = 3;

    let e = &graph.nodes[graph.end];
    frame[e.pos.1][e.pos.0] = 4;
}

struct Graph {
    start: usize,
    end: usize,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    maxx: usize,
    maxy: usize,
}

struct Node {
    pos: Coord,
    dist: usize,
    edges: Vec<usize>,
}

struct Edge {
    tonode: usize,
    indir: Dir,
    outdir: Dir,
    score: u64,
    path: Vec<Coord>,
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

    let dist = |pos: Coord| -> usize { pos.0.abs_diff(epos.0) + pos.1.abs_diff(epos.1) };

    for (y, l) in input.iter().enumerate() {
        for (x, t) in l.iter().enumerate() {
            if *t == MapTile::Wall {
                // Skip wall tiles
                continue;
            }

            // Build position tuple
            let pos = (x, y);

            // Get directions from this tile
            let dirs = dirs(input, pos, None);

            // Is a node if more than 2 outward directions or start or end position
            if dirs.len() > 2 || pos == spos || pos == epos {
                nodes.push(Node {
                    pos,
                    dist: dist(pos),
                    edges: Vec::new(),
                });
            }
        }
    }

    // Build coordinate to node map
    let node_map = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.pos, i))
        .collect::<HashMap<_, _>>();

    // Build edges
    let mut edges = Vec::new();

    for n in nodes.iter_mut() {
        // Loop outward directions from this node
        for (dir, mut next) in dirs(input, n.pos, None) {
            let mut cur_dir = dir;
            let mut path = Vec::new();
            let mut score = 1;

            // Add node to path
            path.push(n.pos);

            loop {
                // Add next position to the path
                path.push(next);

                // Get next direction and position from current without backtracking
                let dirs = dirs(input, next, Some(cur_dir.opposite()));

                if dirs.is_empty() {
                    // Dead end
                    break;
                }

                // Arrived at a node?
                if let Some(n2) = node_map.get(&next) {
                    // Yes - add the edge
                    n.edges.push(edges.len());

                    edges.push(Edge {
                        tonode: *n2,
                        indir: dir,
                        outdir: cur_dir,
                        score,
                        path,
                    });

                    break;
                }

                // Update edge score and direction
                score += 1;

                if dirs[0].0 != cur_dir {
                    score += 1000;
                    cur_dir = dirs[0].0;
                }

                // Set new position
                next = dirs[0].1;
            }
        }
    }

    // Get start and end nodes
    let start = *node_map.get(&spos).unwrap();
    let end = *node_map.get(&epos).unwrap();

    Graph {
        start,
        end,
        nodes,
        edges,
        maxx: input[0].len(),
        maxy: input.len(),
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
    /// Returns the opposite direction
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
