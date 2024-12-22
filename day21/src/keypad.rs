use std::{cmp::Ordering, collections::BinaryHeap};

use fxhash::FxHashMap;

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char),
    Action(Action),
}

#[derive(Debug, Clone)]
pub struct KeyPad {
    width: usize,
    height: usize,
    keys: FxHashMap<Coord, Key>,
    coords: FxHashMap<Key, Coord>,
    routes: FxHashMap<(Coord, Coord), Vec<Vec<Action>>>,
}

impl KeyPad {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            keys: FxHashMap::default(),
            coords: FxHashMap::default(),
            routes: FxHashMap::default(),
        }
    }

    pub fn setkey(&mut self, pos: Coord, key: Key) {
        self.keys.insert(pos, key);
        self.coords.insert(key, pos);
    }

    pub fn build_routes(&mut self) {
        for (from_pos, from_key) in &self.keys {
            for (to_pos, to_key) in &self.keys {
                let route = if from_key == to_key {
                    vec![vec![Action::Activate]]
                } else {
                    self.build_key_routes(from_pos, to_pos)
                };

                self.routes.insert((*from_pos, *to_pos), route);
            }
        }
    }

    pub fn routes(&self, from: Key, to: Key) -> &Vec<Vec<Action>> {
        let from = self.coords.get(&from).unwrap();
        let to = self.coords.get(&to).unwrap();

        self.routes.get(&(*from, *to)).unwrap()
    }

    fn build_key_routes(&self, from: &Coord, to: &Coord) -> Vec<Vec<Action>> {
        // Function to calculate manhattan distance from the end point
        let dist = |(x, y)| to.0.abs_diff(x) + to.1.abs_diff(y);

        // initialise work queue
        let mut queue = BinaryHeap::new();

        queue.push(Work {
            coord: *from,
            dist: dist(*from),
            path: vec![],
        });

        // Create visited hashmap
        let mut visited = FxHashMap::default();

        // Best paths
        let mut best_len = usize::MAX;
        let mut best_paths = Vec::new();

        // Process work queue
        while let Some(mut work) = queue.pop() {
            // Reached end point?
            if work.coord == *to {
                // Yes
                work.path.push(Action::Activate);

                match work.path.len().cmp(&best_len) {
                    std::cmp::Ordering::Less => {
                        // Fewer steps - update best
                        best_len = work.path.len();
                        best_paths = vec![work.path];
                    }
                    std::cmp::Ordering::Equal => {
                        // Same steps - add path
                        best_paths.push(work.path);
                    }
                    std::cmp::Ordering::Greater => (),
                }

                continue;
            }

            // Already visited?
            if let Some(steps) = visited.get_mut(&work.coord) {
                // Yes - visited in fewer steps?
                match work.path.len().cmp(steps) {
                    Ordering::Less => {
                        // Update fewest steps
                        *steps = work.path.len();
                    }
                    Ordering::Equal => (),
                    Ordering::Greater => continue,
                }
            } else {
                // No - mark as visited
                visited.insert(work.coord, work.path.len());
            }

            for (next, action) in self.pos_from(work.coord) {
                let mut next_path = work.path.clone();
                next_path.push(action);

                queue.push(Work {
                    coord: next,
                    dist: dist(next),
                    path: next_path,
                });
            }
        }

        best_paths
    }

    const DIRS: [([isize; 2], Action); 4] = [
        ([1, 0], Action::Right),
        ([-1, 0], Action::Left),
        ([0, 1], Action::Down),
        ([0, -1], Action::Up),
    ];

    fn pos_from(&self, (x, y): Coord) -> impl Iterator<Item = (Coord, Action)> {
        Self::DIRS.iter().filter_map(move |([dx, dy], action)| {
            match x.checked_add_signed(*dx) {
                Some(nx) if nx < self.width => match y.checked_add_signed(*dy) {
                    Some(ny) if ny < self.height => {
                        if self.keys.contains_key(&(nx, ny)) {
                            return Some(((nx, ny), *action));
                        }
                    }
                    _ => (),
                },
                _ => (),
            }

            None
        })
    }
}

#[derive(PartialEq, Eq)]
struct Work {
    coord: Coord,
    dist: usize,
    path: Vec<Action>,
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
