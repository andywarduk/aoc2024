use std::error::Error;

use aoc::input::parse_input;
use graph::Graph;

mod graph;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let graph = parse_input(23, parse_input_str)?;

    // Run parts
    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));

    Ok(())
}

fn part1(graph: &Graph) -> u64 {
    let mut count = 0;

    // Walk the graph finding sets of interconnected nodes
    graph.walk(&mut |set| {
        // Got a set of three?
        if set.len() == 3 {
            // Yes - check if any start with 't'
            if set.iter().any(|&n| graph.node_name(n).starts_with('t')) {
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
    // Get maximum cliques for the graph
    let max_cliques = graph.max_cliques();

    // Should only be one
    assert_eq!(max_cliques.len(), 1);

    // Return separated by ,
    max_cliques[0].join(",")
}

// Input parsing

fn parse_input_str(input: &str) -> Graph {
    // Create new graph
    let mut graph = Graph::default();

    // Process each line of the input
    for line in input.lines() {
        let mut nodes = line.split('-');
        graph.add_edge(nodes.next().unwrap(), nodes.next().unwrap());
    }

    graph
}

#[cfg(test)]
mod tests;
