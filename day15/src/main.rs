use std::{collections::HashSet, error::Error};

use aoc::input::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(15)?;
    let (map, moves) = parse_input(&input);

    // Run parts
    println!("Part 1: {}", part1(map.clone(), &moves));
    println!("Part 2: {}", part2(map.clone(), &moves));

    Ok(())
}

fn part1(mut map: Map, moves: &[Move]) -> u64 {
    for m in moves {
        let (nx, ny) = m.coord(map.robot.0, map.robot.1);

        if make_move(&mut map, m, nx, ny) {
            map.robot = (nx, ny);
        }

        //        println!("{}", map);
    }

    map.boxes.iter().map(|(x, y)| (100 * y) + x).sum::<usize>() as u64
}

fn make_move(map: &mut Map, m: &Move, x: usize, y: usize) -> bool {
    if map.walls.contains(&(x, y)) {
        return false;
    }

    if map.boxes.contains(&(x, y)) {
        let (nx, ny) = m.coord(x, y);

        if !make_move(map, m, nx, ny) {
            return false;
        }

        map.boxes.remove(&(x, y));
        map.boxes.insert((nx, ny));
    }

    true
}

fn part2(mut map: Map, moves: &[Move]) -> u64 {
    0 // TODO
}

type Coord = (usize, usize);

#[derive(Clone)]
struct Map {
    w: usize,
    h: usize,
    walls: HashSet<Coord>,
    boxes: HashSet<Coord>,
    robot: Coord,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.walls.contains(&(x, y)) {
                    write!(f, "#")?;
                } else if self.boxes.contains(&(x, y)) {
                    write!(f, "O")?;
                } else if self.robot == (x, y) {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

enum Move {
    N,
    E,
    S,
    W,
}

impl Move {
    fn coord(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Move::N => (x, y - 1),
            Move::E => (x + 1, y),
            Move::S => (x, y + 1),
            Move::W => (x - 1, y),
        }
    }
}
// Input parsing

fn parse_input(input: &str) -> (Map, Vec<Move>) {
    let mut sections = input.split("\n\n");

    let map = sections.next().unwrap();

    let mut w: usize = 0;
    let mut h: usize = 0;
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = (0, 0);

    map.lines().enumerate().for_each(|(y, l)| {
        if y == 0 {
            w = l.len()
        }
        h += 1;

        l.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                walls.insert((x, y));
            }
            '@' => {
                robot = (x, y);
            }
            'O' => {
                boxes.insert((x, y));
            }
            _ => (),
        })
    });

    let map = Map {
        w,
        h,
        walls,
        boxes,
        robot,
    };

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

    #[test]
    fn test1() {
        let (map, moves) = parse_input(EXAMPLE1);

        assert_eq!(part1(map.clone(), &moves), 10092);
    }

    #[test]
    fn test2() {
        let (map, moves) = parse_input(EXAMPLE2);

        assert_eq!(part1(map.clone(), &moves), 2028);
    }
}
