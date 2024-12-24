use std::error::Error;

use aoc::input::parse_input_vec;
use graph::Graph;

mod graph;

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
    graph.walk(&mut |set| {
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
    let mut largest_set: Vec<&str> = Vec::new();

    // Walk the graph finding sets of interconnected nodes
    graph.walk(&mut |set| {
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
        graph.add_edge(&line.c1, &line.c2);
    }

    graph
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
