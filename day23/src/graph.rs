use fxhash::{FxHashMap, FxHashSet};

// Graph structure
#[derive(Debug, Default)]
pub struct Graph {
    nodes: Vec<String>,
    node_elem: FxHashMap<String, usize>,
    edges: FxHashMap<usize, FxHashSet<usize>>,
}

impl Graph {
    /// Add an edge to the graph
    pub fn add_edge(&mut self, n1: &str, n2: &str) {
        // Insert node 1
        let n1e = if let Some(e) = self.node_elem.get(n1) {
            *e
        } else {
            self.nodes.push(n1.to_string());
            let e = self.nodes.len() - 1;
            self.node_elem.insert(n1.to_string(), e);
            e
        };

        // Insert node 2
        let n2e = if let Some(e) = self.node_elem.get(n2) {
            *e
        } else {
            self.nodes.push(n2.to_string());
            let e = self.nodes.len() - 1;
            self.node_elem.insert(n2.to_string(), e);
            e
        };

        // Insert node 1 -> node 2 edge
        self.edges.entry(n1e).or_default().insert(n2e);

        // Insert node 2 -> node 1 edge
        self.edges.entry(n2e).or_default().insert(n1e);
    }

    /// Returns the node name for a given node ID
    pub fn node_name(&self, elem: usize) -> &str {
        &self.nodes[elem]
    }

    /// Walks cliques in the graph calling a callback for all cliques found
    pub fn walk<F>(&self, cb: &mut F)
    where
        F: FnMut(&FxHashSet<usize>) -> bool,
    {
        // Create empty node set
        let mut node_set = FxHashSet::default();

        // Process each node
        for node in self.node_elem.values() {
            // Recurse
            self.walk_iter(cb, *node, &mut node_set);
        }
    }

    fn walk_iter<F>(&self, cb: &mut F, node: usize, set: &mut FxHashSet<usize>)
    where
        F: FnMut(&FxHashSet<usize>) -> bool,
    {
        // Get node edges
        let edges = self.edges.get(&node).unwrap();

        if set.len() > 1 {
            // Make sure this node is connected to all previous
            for node in set.iter() {
                if !edges.contains(node) {
                    return;
                }
            }
        }

        // Add this node to the set
        set.insert(node);

        if cb(set) {
            // Process edges from this node
            for next in edges {
                // Is the next node greater?
                if *next < node {
                    // No - skip
                    continue;
                }

                // Recurse
                self.walk_iter(cb, *next, set);
            }
        }

        // Remove this node from the set
        set.remove(&node);
    }

    /// Returns a vector of the maximum cliques in the graph
    pub fn max_cliques(&self) -> Vec<Vec<String>> {
        let mut max_len = 0;
        let mut max_sets = Vec::new();

        self.bron_kerbosch(&mut |set| match (set.len()).cmp(&max_len) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => max_sets.push(set.clone()),
            std::cmp::Ordering::Greater => {
                max_sets = vec![set.clone()];
                max_len = set.len();
            }
        });

        max_sets
            .iter()
            .map(|set| {
                let mut vec = set
                    .iter()
                    .map(|ne| self.node_name(*ne).to_string())
                    .collect::<Vec<_>>();
                vec.sort();
                vec
            })
            .collect()
    }

    /// Bron Kerbosch algorithm start
    fn bron_kerbosch<F>(&self, cb: &mut F)
    where
        F: FnMut(&FxHashSet<usize>),
    {
        // Set up initial sets
        let mut r = FxHashSet::default();
        let p = self.node_elem.values().cloned().collect::<FxHashSet<_>>();
        let x = FxHashSet::default();

        // Call recursive function
        self.bron_kerbosch_iter(&mut r, p, x, cb)
    }

    /// Bron Kerbosch algorithm
    fn bron_kerbosch_iter<F>(
        &self,
        r: &mut FxHashSet<usize>,
        mut p: FxHashSet<usize>,
        mut x: FxHashSet<usize>,
        cb: &mut F,
    ) where
        F: FnMut(&FxHashSet<usize>),
    {
        // P and X both empty?
        if p.is_empty() && x.is_empty() {
            // Yes - report set
            cb(r);
        } else {
            // Choose pivot node (node with max connections)
            let empty = FxHashSet::default();

            let (_, pivotset) = p
                .iter()
                .chain(x.iter())
                .fold((0, &empty), |(max, set), node| {
                    // Get edges for this node
                    let edges = self.edges.get(node).unwrap();

                    // More than we've seen before?
                    if edges.len() > max {
                        // Yes - return new max and edges
                        (edges.len(), edges)
                    } else {
                        // No - return old max and edges
                        (max, set)
                    }
                });

            // Iterate all nodes in P
            for node in p.clone() {
                if pivotset.contains(&node) {
                    // This is a node from the pivot node - ignore
                    continue;
                }

                // Get edges for this node
                let edges = self.edges.get(&node).unwrap();

                // Insert this node in to R
                r.insert(node);

                // Set up new P
                let newp = edges.intersection(&p).copied().collect();

                // Set up new X
                let newx = edges.intersection(&x).copied().collect();

                // Call function recursively
                self.bron_kerbosch_iter(r, newp, newx, cb);

                // Put R back
                r.remove(&node);

                // Remove node from P
                p.remove(&node);

                // Add node to X
                x.insert(node);
            }
        }
    }
}
