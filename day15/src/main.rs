use std::{collections::HashMap, error::Error};

use aoc::input::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(15)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    let (mut map, moves) = parse_input(input, false);

    make_moves(&mut map, moves);

    map.items
        .iter()
        .filter(|&(_, i)| *i == Item::Box)
        .map(|(&(x, y), _)| (100 * y) + x)
        .sum::<usize>() as u64
}

fn part2(input: &str) -> u64 {
    let (mut map, moves) = parse_input(input, true);

    make_moves(&mut map, moves);

    map.items
        .iter()
        .filter(|&(_, i)| *i == Item::BoxL)
        .map(|(&(x, y), _)| (100 * y) + x)
        .sum::<usize>() as u64
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

type Coord = (usize, usize);

#[derive(Clone, PartialEq)]
enum Item {
    Wall,
    Box,
    BoxL,
    BoxR,
}

#[derive(Clone)]
struct Map {
    w: usize,
    h: usize,
    items: HashMap<Coord, Item>,
    robot: Coord,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                match self.items.get(&(x, y)) {
                    Some(item) => match item {
                        Item::Wall => write!(f, "#")?,
                        Item::Box => write!(f, "O")?,
                        Item::BoxL => write!(f, "[")?,
                        Item::BoxR => write!(f, "]")?,
                    },
                    None => {
                        if (x, y) == self.robot {
                            write!(f, "@")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
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

fn parse_input(input: &str, double: bool) -> (Map, Vec<Move>) {
    let mut sections = input.split("\n\n");

    let map = sections.next().unwrap();

    let mut w: usize = 0;
    let mut h: usize = 0;
    let mut items = HashMap::new();
    let mut robot = (0, 0);

    map.lines().enumerate().for_each(|(y, l)| {
        if y == 0 {
            if double {
                w = l.len() * 2;
            } else {
                w = l.len();
            }
        }

        h += 1;

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

    let map = Map { w, h, items, robot };

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
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    const EXAMPLE2: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const EXAMPLE3: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[test]
    fn test1() {
        assert_eq!(part1(EXAMPLE1), 10092);
    }

    #[test]
    fn test2() {
        assert_eq!(part1(EXAMPLE2), 2028);
    }

    #[test]
    fn test3() {
        assert_eq!(part2(EXAMPLE1), 9021);
    }

    #[test]
    fn test4() {
        assert_eq!(part2(EXAMPLE3), 618);
    }
}
