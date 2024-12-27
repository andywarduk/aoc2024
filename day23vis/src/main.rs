use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_vec;
use graph::Graph;
use hsl::HSL;

mod graph;

// Convert with: neato -x -Goverlap=false -Tsvg day23.dot -o day23.svg

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(23, input_transform)?;
    let graph = build_graph(input);

    let mut triplets = HashMap::new();

    // Walk the graph finding sets of interconnected nodes
    graph.walk(&mut |set| {
        // Got a set of three?
        if set.len() == 3 {
            // Yes - check if any start with 't'
            if set.iter().any(|n| n.starts_with('t')) {
                // Yes - highlight
                println!("{set:?}");

                for item in set {
                    *triplets.entry(item.to_string()).or_insert(0u64) += 1;
                }
            }

            false
        } else {
            true
        }
    });

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

    let mut highlights: HashMap<String, String> = HashMap::new();

    let max = *triplets.values().max().unwrap();

    let colgap = 180.0 / max as f64;

    for (item, count) in triplets.into_iter() {
        let hsl = HSL {
            h: 30.0 + ((max - count) as f64 * colgap),
            s: 1.0,
            l: 0.5,
        };

        let (r, g, b) = hsl.to_rgb();

        let colour = format!("#{r:02x}{g:02x}{b:02x}");

        highlights.insert(item.to_string(), colour.clone());
    }

    println!("{largest_set:?}");

    for node in largest_set {
        if let Some(colour) = highlights.get_mut(node) {
            *colour = "red".to_string();
        } else {
            highlights.insert(node.to_string(), "red".to_string());
        }
    }

    graph.dump("vis/day23.dot", &highlights)?;

    Ok(())
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
