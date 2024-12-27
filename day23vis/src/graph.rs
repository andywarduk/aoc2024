use std::{collections::HashMap, error::Error, fmt::Write, fs::write};

use fxhash::{FxHashMap, FxHashSet};

#[derive(Debug, Default)]
pub struct Graph {
    nodes: FxHashSet<String>,
    edges: FxHashMap<String, FxHashSet<String>>,
}

impl Graph {
    pub fn add_edge(&mut self, n1: &str, n2: &str) {
        // Insert node 1
        self.nodes.insert(n1.to_string());

        // Insert node 1 edges
        self.edges
            .entry(n1.to_string())
            .or_default()
            .insert(n2.to_string());

        // Insert node 2
        self.nodes.insert(n2.to_string());

        // Insert node 2 edges
        self.edges
            .entry(n2.to_string())
            .or_default()
            .insert(n1.to_string());
    }

    pub fn walk<'a, F>(&'a self, cb: &mut F)
    where
        F: FnMut(&Vec<&'a str>) -> bool,
    {
        // Create empty node set
        let mut node_set = Vec::new();

        // Process each node
        for node in &self.nodes {
            // Recurse
            self.walk_iter(cb, node, &mut node_set);
        }
    }

    fn walk_iter<'a, F>(&'a self, cb: &mut F, node: &'a str, set: &mut Vec<&'a str>)
    where
        F: FnMut(&Vec<&'a str>) -> bool,
    {
        // Get node edges
        let edges = self.edges.get(node).unwrap();

        if set.len() > 1 {
            // Make sure this node is connected to all previous
            for node in set.iter() {
                if !edges.contains(*node) {
                    return;
                }
            }
        }

        // Add this node to the set
        set.push(node);

        if cb(set) {
            // Process edges from this node
            for next in edges {
                // Is the next node greater alphabetically?
                if next.as_str() < node {
                    // No - skip
                    continue;
                }

                // Recurse
                self.walk_iter(cb, next, set);
            }
        }

        // Remove this node from the set
        set.pop();
    }

    pub fn dump(
        &self,
        file: &str,
        highlights: &HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut dotfile = String::new();

        dotfile.write_str("graph {\n")?;
        dotfile.write_str("  fontname=\"Helvetica,Arial,sans-serif\";\n")?;
        dotfile.write_str("  node [fontname=\"Helvetica,Arial,sans-serif\"];\n")?;
        dotfile.write_str("  edge [fontname=\"Helvetica,Arial,sans-serif\"];\n")?;
        dotfile.write_str("\n")?;

        // Draw nodes
        for node in &self.nodes {
            // Work out colour
            let colour = if let Some(colour) = highlights.get(node) {
                colour
            } else {
                "white"
            };

            // Write node for gate
            dotfile.write_fmt(format_args!(
                "    {node} [shape=\"circle\" style=\"filled\" fillcolor=\"{colour}\"];\n",
            ))?;
        }

        for (from, tos) in &self.edges {
            for to in tos {
                dotfile.write_fmt(format_args!("  {} -- {} [color=\"#88f\"];\n", from, to))?;
            }
        }

        // End graph
        dotfile.write_str("}\n")?;

        // Write dot file
        write(file, dotfile)?;

        Ok(())
    }
}
