use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(6, input_transform)?;

    // Run parts
    let (p1, p2) = run_parts(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

fn run_parts(input: &[BoardLine]) -> (u64, u64) {
    // Get board dimensions
    let board_dim = Coord {
        x: input[0].len(),
        y: input.len(),
    };

    // Get guard position
    let guard_pos = guard_pos(input);

    // Walk guard's path
    let path = walk_path(input, &guard_pos, &board_dim);

    // Run parts
    (part1(&path), part2(input, &board_dim, &path))
}

fn part1(path: &[GuardState]) -> u64 {
    // Return length of the path
    let positions = path
        .iter()
        .map(|s| s.pos.clone())
        .collect::<FxHashSet<Coord>>();

    positions.len() as u64
}

fn part2(input: &[BoardLine], board_dim: &Coord, path: &[GuardState]) -> u64 {
    // Pointer to last state
    let mut last_state = &path[0];

    // Set up state hashsets
    let mut turns = FxHashSet::with_capacity_and_hasher(path.len(), Default::default());
    turns.insert(last_state.clone());

    let mut add_turns = FxHashSet::with_capacity_and_hasher(path.len(), Default::default());

    // Set up tried position hash set
    let mut tried = FxHashSet::with_capacity_and_hasher(path.len(), Default::default());
    tried.insert(path[0].pos.clone());

    // Block each untried space on the path and check if a loop occurs
    path.iter()
        .skip(1)
        .filter(|&state| {
            let mut looped = false;

            if !tried.contains(&state.pos) {
                // Mark as tried
                tried.insert(state.pos.clone());

                // Check if a loop occurs
                looped = loop_check(
                    input,
                    board_dim,
                    last_state.clone(),
                    &state.pos,
                    &turns,
                    &mut add_turns,
                );

                if state.dir != last_state.dir {
                    // Add this turn to the turns set
                    turns.insert(last_state.clone());
                }
            }

            // Update last state pointer
            last_state = state;

            looped
        })
        .count() as u64
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct GuardState {
    pos: Coord,
    dir: Dir,
}

fn walk_path(input: &[BoardLine], guard_pos: &Coord, board_dim: &Coord) -> Vec<GuardState> {
    // Set up initial guard state
    let mut guard_state = GuardState {
        pos: guard_pos.clone(),
        dir: Dir::N,
    };

    // Set up path
    let mut visited = Vec::new();
    visited.push(guard_state.clone());

    // Loop next guard positions
    while let Some(next) = guard_state.dir.next_pos(&guard_state.pos, board_dim) {
        if matches!(input[next.y][next.x], Space::Blocked) {
            // Blocked - turn right
            guard_state.dir.rotate_right();
        } else {
            // Set new position
            guard_state.pos = next;

            // Record guard state
            visited.push(guard_state.clone());
        }
    }

    visited
}

fn loop_check(
    input: &[BoardLine],
    board_dim: &Coord,
    mut guard_state: GuardState,
    block_pos: &Coord,
    turns: &FxHashSet<GuardState>,
    add_turns: &mut FxHashSet<GuardState>,
) -> bool {
    // Clear additional states hashset
    add_turns.clear();

    // Get next position
    while let Some(next) = guard_state.dir.next_pos(&guard_state.pos, board_dim) {
        // Blocked?
        if matches!(input[next.y][next.x], Space::Blocked) || &next == block_pos {
            // Yes - turn right
            guard_state.dir.rotate_right();

            // Seen this turn before?
            if turns.contains(&guard_state) || add_turns.contains(&guard_state) {
                // Yes - there is a loop
                return true;
            }

            // No - add this turn
            add_turns.insert(guard_state.clone());
        } else {
            // No - update guard position
            guard_state.pos = next;
        }
    }

    false
}

fn guard_pos(input: &[BoardLine]) -> Coord {
    input
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == Space::Guard { Some(x) } else { None })
                .map(|x| Coord { x, y })
        })
        .expect("Unable to find the guard")
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq)]
enum Space {
    Blocked,
    Empty,
    Guard,
}

type BoardLine = Vec<Space>;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn next_pos(&self, guard_pos: &Coord, board_dim: &Coord) -> Option<Coord> {
        let add = |p, max| {
            let p = p + 1;
            if p == max { None } else { Some(p) }
        };

        let sub = |p| {
            if p == 0 { None } else { Some(p - 1) }
        };

        let (x, y) = match self {
            Dir::N => (guard_pos.x, sub(guard_pos.y)?),
            Dir::E => (add(guard_pos.x, board_dim.x)?, guard_pos.y),
            Dir::S => (guard_pos.x, add(guard_pos.y, board_dim.y)?),
            Dir::W => (sub(guard_pos.x)?, guard_pos.y),
        };

        Some(Coord { x, y })
    }

    fn rotate_right(&mut self) {
        match self {
            Dir::N => *self = Dir::E,
            Dir::E => *self = Dir::S,
            Dir::S => *self = Dir::W,
            Dir::W => *self = Dir::N,
        }
    }
}

// Input parsing

fn input_transform(line: String) -> BoardLine {
    line.chars()
        .map(|c| match c {
            '.' => Space::Empty,
            '#' => Space::Blocked,
            '^' => Space::Guard,
            _ => panic!("Invalid board char {c}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        let (p1, p2) = run_parts(&input);

        assert_eq!(p1, 41);
        assert_eq!(p2, 6);
    }
}
