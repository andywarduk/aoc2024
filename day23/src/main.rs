use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::{FxHashMap, FxHashSet};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(23, input_transform)?;
    let graph = build_graph(input);

    // Run parts
    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));

    Ok(())
}

fn part1(graph: &Graph) -> u64 {
    let mut tset = FxHashSet::default();

    walk_graph(graph, &mut |set| {
        if set.len() == 3 {
            if set.iter().any(|n| n.starts_with('t')) {
                tset.insert(set.clone());
            }

            false
        } else {
            true
        }
    });

    tset.len() as u64
}

fn part2(graph: &Graph) -> String {
    let mut largest_len = 0;
    let mut largest_set = Vec::new();

    walk_graph(graph, &mut |set| {
        if set.len() > largest_len {
            largest_len = set.len();
            largest_set = set.clone();
        }

        true
    });

    largest_set.join(",")
}

fn build_graph(input: Vec<InputEnt>) -> Graph {
    let mut graph = Graph::default();

    for line in input {
        graph.nodes.insert(line.c1.clone());
        graph
            .edges
            .entry(line.c1.clone())
            .or_insert_with(FxHashSet::default)
            .insert(line.c2.clone());

        graph.nodes.insert(line.c2.clone());
        graph
            .edges
            .entry(line.c2)
            .or_insert_with(FxHashSet::default)
            .insert(line.c1);
    }

    graph
}

fn walk_graph<F>(graph: &Graph, cb: &mut F)
where
    F: FnMut(&Vec<String>) -> bool,
{
    for node in &graph.nodes {
        walk_graph_iter(graph, cb, node, Vec::new());
    }
}

fn walk_graph_iter<F>(graph: &Graph, cb: &mut F, node: &str, set: Vec<String>)
where
    F: FnMut(&Vec<String>) -> bool,
{
    let edges = graph.edges.get(node).unwrap();

    if set.len() > 1 {
        // Make sure this node is connected to all previous
        for node in &set {
            if !edges.contains(node) {
                return;
            }
        }
    }

    let mut next_set = set.clone();
    next_set.push(node.to_string());

    if !cb(&next_set) {
        return;
    }

    // Process edges from this node
    for next in edges {
        if next.as_str() < node {
            continue;
        }

        // Recurse
        walk_graph_iter(graph, cb, next, next_set.clone());
    }
}

#[derive(Debug, Default)]
struct Graph {
    nodes: FxHashSet<String>,
    edges: FxHashMap<String, FxHashSet<String>>,
}

struct InputEnt {
    c1: String,
    c2: String,
}

// Input parsing

fn input_transform(line: String) -> InputEnt {
    let mut comps = line.split('-');

    InputEnt {
        c1: comps.next().unwrap().to_string(),
        c2: comps.next().unwrap().to_string(),
    }
}

#[cfg(test)]
mod tests;
