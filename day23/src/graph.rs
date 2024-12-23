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

    pub fn walk<F>(&self, cb: &mut F)
    where
        F: FnMut(&Vec<String>) -> bool,
    {
        // Create empty node set
        let mut node_set = Vec::new();

        // Process each node
        for node in &self.nodes {
            // Recurse
            self.walk_iter(cb, node, &mut node_set);
        }
    }

    fn walk_iter<F>(&self, cb: &mut F, node: &str, set: &mut Vec<String>)
    where
        F: FnMut(&Vec<String>) -> bool,
    {
        // Get node edges
        let edges = self.edges.get(node).unwrap();

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
                self.walk_iter(cb, next, set);
            }
        }

        // Remove this node from the set
        set.pop();
    }
}
