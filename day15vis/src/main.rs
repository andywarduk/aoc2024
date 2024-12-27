use std::{collections::HashMap, error::Error};

use aoc::{
    gif::{Gif, IdenticalAction},
    input::parse_input,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input(15, |s| s.to_string())?;

    // Run parts
    render(&input, "vis/day15-1.gif", false)?;
    render(&input, "vis/day15-2.gif", true)?;

    Ok(())
}

const PALETTE: [[u8; 3]; 4] = [[0, 0, 0], [255, 64, 64], [128, 128, 255], [255, 255, 64]];

const CELLSIZE: usize = 12;

fn render(input: &str, file: &str, double: bool) -> Result<(), Box<dyn Error>> {
    let (mut map, moves) = parse_input_str(input, double);

    let mut gif = Gif::new(
        file,
        &PALETTE,
        (map.w * CELLSIZE) as u16,
        (map.h * CELLSIZE) as u16,
        1,
        1,
    )?;

    make_moves(&mut gif, &mut map, moves)?;

    gif.delay(500)?;

    Ok(())
}

fn make_moves(gif: &mut Gif, map: &mut Map, moves: Vec<Move>) -> Result<(), Box<dyn Error>> {
    draw_map(gif, map)?;

    for m in moves {
        let robot_next = m.coord(&map.robot);

        let mut next_moves = Vec::new();

        if check_move(map, &m, robot_next, &mut next_moves) {
            apply_moves(map, &next_moves);

            map.robot = robot_next;

            if !next_moves.is_empty() {
                draw_map(gif, map)?;
            }
        }
    }

    draw_map(gif, map)?;

    Ok(())
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

fn apply_moves(map: &mut Map, moves: &Vec<(Coord, Coord)>) {
    for (from, to) in moves {
        let item = map.items.remove(from).unwrap();
        map.items.insert(*to, item);
    }
}

fn draw_map(gif: &mut Gif, map: &Map) -> Result<(), Box<dyn Error>> {
    let mut frame = gif.empty_frame();

    let mut draw = |bitmap: &[[u8; CELLSIZE]; CELLSIZE], x: usize, y: usize, colour: u8| {
        let gx = x * CELLSIZE;
        let gy = y * CELLSIZE;

        for y in 0..CELLSIZE {
            for x in 0..CELLSIZE {
                if bitmap[y][x] == 1 {
                    frame[gy + y][gx + x] = colour;
                }
            }
        }
    };

    for ((x, y), item) in map.items.iter() {
        let (bitmap, colour) = match item {
            Item::Wall => (&WALL, 1),
            Item::Box => (&BOX, 3),
            Item::BoxL => (&BOXL, 3),
            Item::BoxR => (&BOXR, 3),
        };

        draw(bitmap, *x, *y, colour);
    }

    let (x, y) = map.robot;
    draw(&ROBOT, x, y, 2);

    gif.draw_frame_identical_check(frame, 2, IdenticalAction::Ignore)?;

    Ok(())
}

const WALL: [[u8; CELLSIZE]; CELLSIZE] = [
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1],
    [1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1],
    [1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const BOX: [[u8; CELLSIZE]; CELLSIZE] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0],
    [0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0],
    [0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0],
    [0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0],
    [0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const BOXL: [[u8; CELLSIZE]; CELLSIZE] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
    [0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1],
    [0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const BOXR: [[u8; CELLSIZE]; CELLSIZE] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0],
    [1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0],
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0],
    [1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const ROBOT: [[u8; CELLSIZE]; CELLSIZE] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0],
    [0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0],
    [0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

type Coord = (usize, usize);

#[derive(PartialEq)]
enum Item {
    Wall,
    Box,
    BoxL,
    BoxR,
}

struct Map {
    w: usize,
    h: usize,
    items: HashMap<Coord, Item>,
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
