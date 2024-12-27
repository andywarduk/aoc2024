use std::error::Error;

use aoc::input::parse_input;
use fxhash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input(15, |s| s.to_string())?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    let (mut map, moves) = parse_input_str(input, false);

    make_moves(&mut map, moves);
    calc_gps(&map, Item::Box)
}

fn part2(input: &str) -> u64 {
    let (mut map, moves) = parse_input_str(input, true);

    make_moves(&mut map, moves);
    calc_gps(&map, Item::BoxL)
}

fn make_moves(map: &mut Map, moves: Vec<Move>) {
    for m in moves {
        let robot_next = m.coord(&map.robot);

        let mut next_moves = Vec::new();

        if check_move(map, &m, robot_next, &mut next_moves) {
            apply_moves(map, next_moves);

            map.robot = robot_next;
        }
    }
}

fn check_move(map: &Map, m: &Move, from: Coord, next_moves: &mut Vec<(Coord, Coord)>) -> bool {
    let updown = *m == Move::N || *m == Move::S;

    let mut check_next: Vec<(Coord, Coord)> = Vec::new();

    if !match map.items.get(&from) {
        Some(Item::Wall) => false,
        Some(Item::Box) => {
            let to = m.coord(&from);
            check_next.push((from, to));
            true
        }
        Some(Item::BoxL) => {
            let to = m.coord(&from);
            if updown {
                check_next.push(((from.0 + 1, from.1), (to.0 + 1, to.1)));
            }
            check_next.push((from, to));
            true
        }
        Some(Item::BoxR) => {
            let to = m.coord(&from);
            if updown {
                check_next.push(((from.0 - 1, from.1), (to.0 - 1, to.1)));
            }
            check_next.push((from, to));
            true
        }
        None => true,
    } {
        // Move not possible
        return false;
    }

    if check_next.is_empty() {
        true
    } else {
        check_next.iter().all(|ent| {
            if !next_moves.contains(ent) {
                if check_move(map, m, ent.1, next_moves) {
                    next_moves.push(*ent);
                    true
                } else {
                    false
                }
            } else {
                true
            }
        })
    }
}

fn apply_moves(map: &mut Map, moves: Vec<(Coord, Coord)>) {
    for (from, to) in moves {
        let item = map.items.remove(&from).unwrap();
        map.items.insert(to, item);
    }
}

fn calc_gps(map: &Map, item: Item) -> u64 {
    map.items
        .iter()
        .filter(|&(_, i)| *i == item)
        .map(|(&(x, y), _)| (100 * y) + x)
        .sum::<usize>() as u64
}

type Coord = (usize, usize);

#[derive(PartialEq)]
enum Item {
    Wall,
    Box,
    BoxL,
    BoxR,
}

struct Map {
    items: FxHashMap<Coord, Item>,
    robot: Coord,
}

#[derive(PartialEq)]
enum Move {
    N,
    E,
    S,
    W,
}

impl Move {
    fn coord(&self, c: &Coord) -> Coord {
        match self {
            Move::N => (c.0, c.1 - 1),
            Move::E => (c.0 + 1, c.1),
            Move::S => (c.0, c.1 + 1),
            Move::W => (c.0 - 1, c.1),
        }
    }
}

// Input parsing

fn parse_input_str(input: &str, double: bool) -> (Map, Vec<Move>) {
    let mut sections = input.split("\n\n");

    let map = sections.next().unwrap();

    let mut items = FxHashMap::default();
    let mut robot = (0, 0);

    map.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                if double {
                    let xd = x * 2;
                    items.insert((xd, y), Item::Wall);
                    items.insert((xd + 1, y), Item::Wall);
                } else {
                    items.insert((x, y), Item::Wall);
                }
            }
            '@' => {
                if double {
                    let xd = x * 2;
                    robot = (xd, y);
                } else {
                    robot = (x, y);
                }
            }
            'O' => {
                if double {
                    let xd = x * 2;
                    items.insert((xd, y), Item::BoxL);
                    items.insert((xd + 1, y), Item::BoxR);
                } else {
                    items.insert((x, y), Item::Box);
                }
            }
            _ => (),
        })
    });

    let map = Map { items, robot };

    let moves = sections
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Move::N),
            '>' => Some(Move::E),
            'v' => Some(Move::S),
            '<' => Some(Move::W),
            _ => None,
        })
        .collect::<Vec<_>>();

    (map, moves)
}

#[cfg(test)]
mod tests;
