use std::{cmp::Ordering, collections::VecDeque};

use fxhash::FxHashMap;

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action {
    Activate,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Key {
    Action(Action),
    Num(u8),
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => f.write_fmt(format_args!("{n}")),
            Self::Action(a) => f.write_fmt(format_args!("{a:?}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyPad {
    coords: FxHashMap<Key, Coord>,
    routes: FxHashMap<(Coord, Coord), Vec<Vec<Key>>>,
}

impl KeyPad {
    pub fn routes(&self, from: Key, to: Key) -> &Vec<Vec<Key>> {
        // Convert keys to coordinates
        let from = self.coords.get(&from).unwrap();
        let to = self.coords.get(&to).unwrap();

        // Get the routes
        self.routes.get(&(*from, *to)).unwrap()
    }
}

pub struct KeyPadBuilder {
    width: usize,
    height: usize,
    keys: FxHashMap<Coord, Key>,
    coords: FxHashMap<Key, Coord>,
}

impl KeyPadBuilder {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            keys: FxHashMap::default(),
            coords: FxHashMap::default(),
        }
    }

    pub fn setkey(mut self, pos: Coord, key: Key) -> Self {
        // Set key at coordinate
        assert!(pos.0 < self.width && pos.1 < self.height);

        self.keys.insert(pos, key);
        self.coords.insert(key, pos);

        self
    }

    pub fn build(self) -> KeyPad {
        let mut routes = FxHashMap::default();

        // Loop each position
        for from_pos in self.keys.keys() {
            // Loop each position
            for to_pos in self.keys.keys() {
                // Calculate routes from key to key
                routes.insert(
                    (*from_pos, *to_pos),
                    self.build_key_routes(from_pos, to_pos),
                );
            }
        }

        let mut coords = self.coords;
        coords.shrink_to_fit();

        routes.shrink_to_fit();

        KeyPad { coords, routes }
    }

    fn build_key_routes(&self, from: &Coord, to: &Coord) -> Vec<Vec<Key>> {
        // Initialise work queue
        let mut queue = VecDeque::new();

        queue.push_back(BuildWork {
            coord: *from,
            dir: None,
            steps: 0,
            dir_changes: 0,
            path: vec![],
        });

        // Create visited hashmap
        let mut visited = FxHashMap::default();

        // Best paths
        let mut best_len = usize::MAX;
        let mut best_paths = Vec::new();

        // Process work queue
        while let Some(mut work) = queue.pop_front() {
            // Reached end point?
            if work.coord == *to {
                // Yes
                work.path.push(Key::Action(Action::Activate));

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

            // Loop next positions
            for (next, action) in self.pos_from(work.coord) {
                // Direction changed?
                let dir = Some(action);
                let mut dir_changes = work.dir_changes;

                if dir != work.dir {
                    dir_changes += 1;

                    // Only allow up to 2 direction changes
                    if dir_changes > 2 {
                        continue;
                    }
                }

                // Add a step
                let steps = work.steps + 1;

                // Build new path
                let mut path = work.path.clone();
                path.push(Key::Action(action));

                // Add to work queue
                queue.push_back(BuildWork {
                    coord: next,
                    dir,
                    steps,
                    dir_changes,
                    path,
                });
            }
        }

        // Return next paths
        best_paths
    }

    const DIRS: [([isize; 2], Action); 4] = [
        ([-1, 0], Action::Left),
        ([0, -1], Action::Up),
        ([0, 1], Action::Down),
        ([1, 0], Action::Right),
    ];

    fn pos_from(&self, (x, y): Coord) -> impl Iterator<Item = (Coord, Action)> {
        Self::DIRS.iter().filter_map(move |([dx, dy], action)| {
            match x.checked_add_signed(*dx) {
                Some(nx) if nx < self.width => match y.checked_add_signed(*dy) {
                    Some(ny) if ny < self.height => {
                        // Check this coordinate contains a key
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
struct BuildWork {
    coord: Coord,
    dir: Option<Action>,
    steps: u8,
    dir_changes: u8,
    path: Vec<Key>,
}
