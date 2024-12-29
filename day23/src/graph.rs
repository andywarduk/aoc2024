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

    pub fn max_cliques(&self) -> Vec<Vec<String>> {
        let r = FxHashSet::default();
        let p = self.nodes.iter().cloned().collect::<FxHashSet<_>>();
        let x = FxHashSet::default();

        let mut max_len = 0;
        let mut max_sets = Vec::new();

        self.bron_kerbosch(r, p, x, &mut |set| {
            match (set.len()).cmp(&max_len) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => max_sets.push(set.clone()),
                std::cmp::Ordering::Greater => {
                    max_sets = vec![set.clone()];
                    max_len = set.len();
                }
            }

            true
        });

        max_sets
            .iter()
            .map(|set| {
                let mut vec = set.iter().cloned().collect::<Vec<_>>();
                vec.sort();
                vec
            })
            .collect()
    }

    fn bron_kerbosch<F>(
        &self,
        r: FxHashSet<String>,
        mut p: FxHashSet<String>,
        mut x: FxHashSet<String>,
        cb: &mut F,
    ) -> bool
    where
        F: FnMut(&FxHashSet<String>) -> bool,
    {
        if p.is_empty() && x.is_empty() {
            cb(&r)
        } else {
            // Choose pivot node
            let empty = FxHashSet::default();

            let (_, pivotset) = p
                .iter()
                .chain(x.iter())
                .fold((0, &empty), |(max, set), node| {
                    let edges = self.edges.get(node).unwrap();

                    if edges.len() > max {
                        (edges.len(), edges)
                    } else {
                        (max, set)
                    }
                });

            for node in p.clone() {
                if pivotset.contains(&node) {
                    continue;
                }

                let edges = self.edges.get(&node).unwrap();

                let mut newr = r.clone();
                newr.insert(node.clone());

                let mut newp = edges.clone();
                newp.retain(|n| p.contains(n));

                let mut newx = edges.clone();
                newx.retain(|n| x.contains(n));

                if !self.bron_kerbosch(newr, newp, newx, cb) {
                    return false;
                }

                p.remove(&node);
                x.insert(node);
            }

            true
        }
    }
}
