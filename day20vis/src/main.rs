use std::error::Error;

use aoc::{gif::Gif, input::parse_input_vec};
use fxhash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(20, input_transform)?;

    let pathmap = find_path(&input);

    draw(&input, &pathmap)?;

    Ok(())
}

const SCALE: u16 = 5;

fn draw(map: &[InputEnt], pathmap: &FxHashMap<Coord, usize>) -> Result<(), Box<dyn Error>> {
    let palette = vec![
        [0, 0, 0],       // Black
        [32, 32, 192],   // Blue (walls)
        [128, 128, 128], // Gray (path)
        [255, 255, 255], // White (cheat)
        [255, 0, 0],     // Red (start)
        [0, 255, 0],     // Green (end)
    ];

    let mut gif = Gif::new(
        "vis/day20.gif",
        &palette,
        map[0].len() as u16,
        map.len() as u16,
        SCALE,
        SCALE,
    )?;

    let start = find_tile(map, Tile::Start);
    let end = find_tile(map, Tile::End);

    let draw_walls = |frame: &mut Vec<Vec<u8>>| {
        for (y, l) in map.iter().enumerate() {
            for (x, t) in l.iter().enumerate() {
                if *t == Tile::Wall {
                    frame[y][x] = 1;
                }
            }
        }
    };

    let draw_startend = |frame: &mut Vec<Vec<u8>>| {
        frame[start.1][start.0] = 4;
        frame[end.1][end.0] = 5;
    };

    let draw_line = |frame: &mut Vec<Vec<u8>>, from: Coord, to: Coord, colour: u8| {
        let mut x0 = from.0 as isize;
        let mut y0 = from.1 as isize;
        let x1 = to.0 as isize;
        let y1 = to.1 as isize;

        let dx = x0.abs_diff(x1) as isize;
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y0.abs_diff(y1) as isize);
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        frame[y0 as usize][x0 as usize] = colour;

        loop {
            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * error;

            if e2 >= dy {
                error += dy;
                x0 += sx;
                frame[y0 as usize][x0 as usize] = colour;
            }

            if e2 <= dx {
                error += dx;
                y0 += sy;
                frame[y0 as usize][x0 as usize] = colour;
            }
        }
    };

    // Draw frame with full path
    let mut frame = gif.empty_frame();

    draw_walls(&mut frame);

    for &(x, y) in pathmap.keys() {
        frame[y][x] = 2;
    }

    draw_startend(&mut frame);

    gif.draw_frame(frame, 100)?;

    let mut draw_cheat = |pos, idx, cheat_pos, cheat_idx| -> Result<(), Box<dyn Error>> {
        let mut frame = gif.empty_frame();

        draw_walls(&mut frame);

        for (&(x, y), &i) in pathmap {
            if i <= idx || i >= cheat_idx {
                frame[y][x] = 2;
            }
        }

        draw_line(&mut frame, pos, cheat_pos, 3);

        draw_startend(&mut frame);

        gif.draw_frame(frame, 100)
    };

    // Draw best cheat for 2
    for (pos, idx, cheat_pos, cheat_idx) in best_cheats(map, pathmap, 2) {
        draw_cheat(pos, idx, cheat_pos, cheat_idx)?;
    }

    // Draw best cheat for 20
    for (pos, idx, cheat_pos, cheat_idx) in best_cheats(map, pathmap, 20) {
        draw_cheat(pos, idx, cheat_pos, cheat_idx)?;
    }

    Ok(())
}

fn best_cheats(
    map: &[InputEnt],
    pathmap: &FxHashMap<Coord, usize>,
    duration: usize,
) -> Vec<(Coord, usize, Coord, usize)> {
    let mut best_saved = 0;
    let mut best = Vec::new();

    for (saved, pos, idx, cheat_pos, cheat_idx) in find_cheats(map, pathmap, duration, 2) {
        match saved.cmp(&best_saved) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => best.push((pos, idx, cheat_pos, cheat_idx)),
            std::cmp::Ordering::Greater => {
                best_saved = saved;
                best = vec![(pos, idx, cheat_pos, cheat_idx)];
            }
        }
    }

    best.sort_by(|a, b| b.3.cmp(&a.3).then_with(|| b.1.cmp(&a.1)));

    best
}

fn find_path(map: &[Vec<Tile>]) -> FxHashMap<Coord, usize> {
    let start = find_tile(map, Tile::Start);
    let end = find_tile(map, Tile::End);

    let mut pathmap = FxHashMap::default();

    let mut pos = start;
    let mut n1_pos;
    let mut n2_pos = None;
    let mut idx = 0;

    pathmap.insert(pos, idx);

    while pos != end {
        n1_pos = Some(pos);
        pos = next_pos(map, pos, n2_pos);

        idx += 1;
        pathmap.insert(pos, idx);

        n2_pos = n1_pos;
    }

    pathmap
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn next_pos(map: &[Vec<Tile>], pos: Coord, last_pos: Option<Coord>) -> Coord {
    DIRS.iter()
        .map(|(dx, dy)| (pos.0 as isize + dx, pos.1 as isize + dy))
        .find_map(|(x, y)| {
            let x = x as usize;
            let y = y as usize;
            let next = (x, y);

            if map[y][x] != Tile::Wall {
                if let Some(last) = last_pos {
                    if next != last { Some(next) } else { None }
                } else {
                    Some(next)
                }
            } else {
                None
            }
        })
        .unwrap()
}

fn find_tile(map: &[Vec<Tile>], tile: Tile) -> Coord {
    map.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, t)| if *t == tile { Some((x, y)) } else { None })
        })
        .unwrap()
}

fn find_cheats(
    map: &[Vec<Tile>],
    pathmap: &FxHashMap<Coord, usize>,
    duration: usize,
    cutoff: usize,
) -> impl Iterator<Item = (usize, Coord, usize, Coord, usize)> {
    pathmap.iter().flat_map(move |(&pos, &idx)| {
        (2..=duration).flat_map(move |duration| {
            cheat_jumps(map, pos, duration).filter_map(move |cheat_pos| {
                if let Some(cheat_idx) = pathmap.get(&cheat_pos) {
                    if *cheat_idx > idx && *cheat_idx > duration {
                        let saved = cheat_idx - idx - duration;

                        if saved >= cutoff {
                            return Some((saved, pos, idx, cheat_pos, *cheat_idx));
                        }
                    }
                }

                None
            })
        })
    })
}

fn cheat_jumps(map: &[Vec<Tile>], pos: Coord, duration: usize) -> impl Iterator<Item = Coord> {
    let (x, y) = pos;

    // eg duration = 3
    //    X
    //   X.X
    //  X...X
    // X..P..X
    //  X...X
    //   X.X
    //    X

    let ne = move |i: usize| -> (isize, isize) { ((i as isize), -((duration - i) as isize)) };
    let se = move |i: usize| -> (isize, isize) { ((duration - i) as isize, i as isize) };
    let sw = move |i: usize| -> (isize, isize) { (-(i as isize), (duration - i) as isize) };
    let nw = move |i: usize| -> (isize, isize) { (-((duration - i) as isize), -(i as isize)) };

    (0..duration)
        .map(ne)
        .chain((0..duration).map(se))
        .chain((0..duration).map(sw))
        .chain((0..duration).map(nw))
        .filter_map(move |(dx, dy)| {
            let x = x as isize + dx;
            let y = y as isize + dy;

            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;

                if y < map.len() && x < map[0].len() && map[y][x] != Tile::Wall {
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
        })
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

// Input parsing

type InputEnt = Vec<Tile>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => panic!("Invalid tile"),
        })
        .collect()
}
