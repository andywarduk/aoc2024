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
    let mut count = 0;

    // Walk the graph finding sets of interconnected nodes
    walk_graph(graph, &mut |set| {
        // Got a set of three?
        if set.len() == 3 {
            // Yes - check if any start with 't'
            if set.iter().any(|n| n.starts_with('t')) {
                // Yes - count
                count += 1;
            }

            false
        } else {
            true
        }
    });

    // Return count
    count
}

fn part2(graph: &Graph) -> String {
    let mut largest_len = 0;
    let mut largest_set = Vec::new();

    // Walk the graph finding sets of interconnected nodes
    walk_graph(graph, &mut |set| {
        // Is this set bigger than the biggest we've seen?
        if set.len() > largest_len {
            // Yes - save it
            largest_len = set.len();
            largest_set = set.clone();
        }

        true
    });

    largest_set.join(",")
}

fn build_graph(input: Vec<InputEnt>) -> Graph {
    // Create new graph
    let mut graph = Graph::default();

    // Process each line of the input
    for line in input {
        // Insert node 1
        graph.nodes.insert(line.c1.clone());

        // Insert node 1 edges
        graph
            .edges
            .entry(line.c1.clone())
            .or_insert_with(FxHashSet::default)
            .insert(line.c2.clone());

        // Insert node 2
        graph.nodes.insert(line.c2.clone());

        // Insert node 2 edges
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
    // Create empty node set
    let mut node_set = Vec::new();

    // Process each node
    for node in &graph.nodes {
        // Recurse
        walk_graph_iter(graph, cb, node, &mut node_set);
    }
}

fn walk_graph_iter<F>(graph: &Graph, cb: &mut F, node: &str, set: &mut Vec<String>)
where
    F: FnMut(&Vec<String>) -> bool,
{
    // Get node edges
    let edges = graph.edges.get(node).unwrap();

    if set.len() > 1 {
        // Make sure this node is connected to all previous
        for node in set.iter() {
            if !edges.contains(node) {
                return;
            }
        }
    }

    // Add this node to the set
    set.push(node.to_string());

    if cb(set) {
        // Process edges from this node
        for next in edges {
            // Is the next node greater alphabetically?
            if next.as_str() < node {
                // No - skip
                continue;
            }

            // Recurse
            walk_graph_iter(graph, cb, next, set);
        }
    }

    // Remove this node from the set
    set.pop();
}

#[derive(Debug, Default)]
struct Graph {
    nodes: FxHashSet<String>,
    edges: FxHashMap<String, FxHashSet<String>>,
}

// Input parsing

struct InputEnt {
    c1: String,
    c2: String,
}

fn input_transform(line: String) -> InputEnt {
    let mut comps = line.split('-');

    InputEnt {
        c1: comps.next().unwrap().to_string(),
        c2: comps.next().unwrap().to_string(),
    }
}

#[cfg(test)]
mod tests;
