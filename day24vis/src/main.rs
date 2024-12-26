use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::fs::write;

use aoc::input::read_input_file;

// Convert with: dot -Tsvg -o day24.svg day24.dot

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(24).unwrap();
    let (nodes, edges) = parse_input(&input);

    let mut dotfile = String::new();

    dotfile.write_str("digraph {\n")?;
    dotfile.write_str("  rankdir=\"LR\";\n")?;

    let draw_node = |dotfile: &mut String, i, node: &Node| -> Result<(), Box<dyn Error>> {
        let attrs = match node.ntype {
            NodeType::Input => "shape=\"circle\" style=\"filled\" fillcolor=\"#88ff88\"",
            NodeType::Output => "shape=\"circle\" style=\"filled\" fillcolor=\"#ff8888\"",
            NodeType::Gate => "shape=\"box\" style=\"filled\" fillcolor=\"#8888ff\"",
        };

        dotfile.write_fmt(format_args!("  n{i} [{attrs} label=\"{}\"];\n", node.name))?;

        Ok(())
    };

    for (i, node) in nodes.iter().enumerate() {
        draw_node(&mut dotfile, i, node)?;
    }

    for edge in edges {
        dotfile.write_fmt(format_args!(
            "  n{} -> n{} [label=\"{}\"];\n",
            edge.from, edge.to, edge.name
        ))?;
    }

    dotfile.write_str("}\n")?;

    // Write dot file
    write("vis/day24.dot", dotfile)?;

    Ok(())
}

enum NodeType {
    Input,
    Output,
    Gate,
}

struct Node {
    name: String,
    ntype: NodeType,
}

struct Edge {
    name: String,
    from: usize,
    to: usize,
}

// Input parsing

fn parse_input(input: &str) -> (Vec<Node>, Vec<Edge>) {
    let mut inputs = HashMap::new();
    let mut outputs = HashMap::new();

    let mut nodes = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(num, l)| {
            let mut split = l.split_ascii_whitespace();

            let in1 = split.next().unwrap().to_string();
            let op = split.next().unwrap().to_string();
            let in2 = split.next().unwrap().to_string();
            split.next().unwrap();
            let out = split.next().unwrap().to_string();

            inputs.entry(in1).or_insert_with(Vec::new).push(num);
            inputs.entry(in2).or_insert_with(Vec::new).push(num);

            outputs.insert(out, num);

            Node {
                name: op,
                ntype: NodeType::Gate,
            }
        })
        .collect::<Vec<_>>();

    let mut edges = Vec::new();

    let mut innames = HashMap::new();

    for (wire, to_vec) in &inputs {
        for &to in to_vec {
            // Where is this connected from?
            if let Some(&from) = outputs.get(wire) {
                edges.push(Edge {
                    name: wire.clone(),
                    from,
                    to,
                })
            } else {
                // Must be an input
                let from = match innames.get(wire) {
                    Some(from) => *from,
                    None => {
                        let from = nodes.len();

                        innames.insert(wire.clone(), from);

                        nodes.push(Node {
                            name: wire.clone(),
                            ntype: NodeType::Input,
                        });

                        from
                    }
                };

                edges.push(Edge {
                    name: wire.clone(),
                    from,
                    to,
                });
            }
        }
    }

    for (wire, &from) in &outputs {
        if !inputs.contains_key(wire) {
            // Must be an output
            let to = nodes.len();

            nodes.push(Node {
                name: wire.clone(),
                ntype: NodeType::Output,
            });

            edges.push(Edge {
                name: wire.clone(),
                from,
                to,
            });
        }
    }

    (nodes, edges)
}
