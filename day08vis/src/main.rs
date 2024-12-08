use std::{
    collections::{HashMap, HashSet},
    error::Error,
    ops::Index,
};

use aoc::{gif::Gif, input::parse_input_vec};
use hsl::HSL;

const CELLSIZE: usize = 18;

const ANTENNA: [[u8; CELLSIZE]; CELLSIZE] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const ANTI: [[u8; CELLSIZE]; CELLSIZE] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(8, input_transform)?;

    // Run parts
    part1(&input, "vis/day08-1.gif")?;
    part2(&input, "vis/day08-2.gif")?;

    Ok(())
}

fn part1(input: &[InputEnt], file: &str) -> Result<(), Box<dyn Error>> {
    let positions = get_positions(input);

    let mut intpos: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

    let mut add_pos = |c, x: isize, y: isize, ox, oy| {
        if x >= 0
            && y >= 0
            && x < input[0].len() as isize
            && y < input.len() as isize
            && (x, y) != (ox, oy)
        {
            intpos
                .entry((x as usize, y as usize))
                .and_modify(|m| {
                    m.insert(c);
                })
                .or_insert_with(|| {
                    let mut h = HashSet::new();
                    h.insert(c);
                    h
                });
        }
    };

    for (c, p) in &positions {
        for (i, (x1, y1)) in p.iter().enumerate() {
            let (x1, y1) = (*x1 as isize, *y1 as isize);

            for (x2, y2) in p[i + 1..].iter() {
                let (x2, y2) = (*x2 as isize, *y2 as isize);

                let (xd, yd) = (x2 - x1, y2 - y1);

                add_pos(*c, x1 - xd, y1 - yd, x2, y2);
                add_pos(*c, x1 + xd, y1 + yd, x2, y2);

                add_pos(*c, x2 - xd, y2 - yd, x1, y1);
                add_pos(*c, x2 + xd, y2 + yd, x1, y1);
            }
        }
    }

    let (palette, layout) = build_pic(input, &positions, &intpos);

    draw_pic(file, &palette, &layout)?;

    Ok(())
}

fn part2(input: &[InputEnt], file: &str) -> Result<(), Box<dyn Error>> {
    let positions = get_positions(input);

    let mut intpos: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

    let mut add_pos = |c, x, y, xd, yd| {
        let mut x = x as isize;
        let mut y = y as isize;

        loop {
            intpos
                .entry((x as usize, y as usize))
                .and_modify(|m| {
                    m.insert(c);
                })
                .or_insert_with(|| {
                    let mut h = HashSet::new();
                    h.insert(c);
                    h
                });

            x += xd;
            y += yd;

            if x < 0 || x as usize >= input[0].len() || y < 0 || y as usize >= input.len() {
                break;
            }
        }
    };

    for (c, p) in &positions {
        for (i, (x1, y1)) in p.iter().enumerate() {
            for (x2, y2) in p[i + 1..].iter() {
                let xd = *x2 as isize - *x1 as isize;
                let yd = *y2 as isize - *y1 as isize;

                add_pos(*c, *x1, *y1, -xd, -yd);
                add_pos(*c, *x1, *y1, xd, yd);
            }
        }
    }

    let (palette, layout) = build_pic(input, &positions, &intpos);

    draw_pic(file, &palette, &layout)?;

    Ok(())
}

fn build_pic(
    input: &[InputEnt],
    positions: &HashMap<char, Vec<(usize, usize)>>,
    intpos: &HashMap<(usize, usize), HashSet<char>>,
) -> (Vec<[u8; 3]>, Vec<Vec<Square>>) {
    let mut layout = vec![vec![Square::default(); input[0].len()]; input.len()];

    let mut palette = vec![[0, 0, 0]];

    let mut chars = positions.iter().map(|(c, _)| c).collect::<Vec<_>>();
    chars.sort();

    let colinc = 360.0 / (chars.len() - 1) as f64;

    for i in 0..chars.len() {
        let hsl = HSL {
            h: (i as f64 * colinc).min(360.0),
            s: 1.0,
            l: 0.5,
        };

        let c = hsl.to_rgb();

        palette.push([c.0, c.1, c.2]);
    }

    for (pc, v) in positions {
        for (x, y) in v {
            layout[*y][*x].outer = Some(chars.iter().position(|c| **c == *pc).unwrap());
        }
    }

    for ((x, y), v) in intpos {
        let colours = v
            .iter()
            .map(|pc| chars.iter().position(|c| **c == *pc).unwrap())
            .collect::<Vec<_>>();

        if colours.len() == 1 {
            layout[*y][*x].inner = Some(colours[0]);
        } else {
            let c = colours.iter().fold([0, 0, 0], |acc, c| {
                [
                    acc[0] + palette[c + 1][0] as usize,
                    acc[1] + palette[c + 1][1] as usize,
                    acc[2] + palette[c + 1][2] as usize,
                ]
            });

            let c = [
                (c[0] / colours.len()) as u8,
                (c[1] / colours.len()) as u8,
                (c[2] / colours.len()) as u8,
            ];

            if let Some(pc) = palette.iter().position(|p| *p == c) {
                layout[*y][*x].inner = Some(pc);
            } else {
                layout[*y][*x].inner = Some(palette.len());
                palette.push(c);
            }
        }
    }

    (palette, layout)
}

fn draw_pic(file: &str, palette: &[[u8; 3]], layout: &[Vec<Square>]) -> Result<(), Box<dyn Error>> {
    let mut gif = Gif::new(
        file,
        palette,
        (layout[0].len() * CELLSIZE) as u16,
        (layout.len() * CELLSIZE) as u16,
        1,
        1,
    )?;

    let mut frame = gif.empty_frame();

    for (y, l) in layout.iter().enumerate() {
        let gy = y * CELLSIZE;

        for (x, c) in l.iter().enumerate() {
            let gx = x * CELLSIZE;

            if let Some(col) = c.outer {
                for y in 0..CELLSIZE {
                    for x in 0..CELLSIZE {
                        if ANTENNA[y][x] != 0 {
                            frame[gy + y][gx + x] = col as u8;
                        }
                    }
                }
            }

            if let Some(col) = c.inner {
                for y in 0..CELLSIZE {
                    for x in 0..CELLSIZE {
                        if ANTI[y][x] != 0 {
                            frame[gy + y][gx + x] = col as u8;
                        }
                    }
                }
            }
        }
    }
    gif.draw_frame(frame, 0)?;

    Ok(())
}

#[derive(Default, Clone)]
struct Square {
    outer: Option<usize>,
    inner: Option<usize>,
}

// Input parsing

type InputEnt = Vec<char>;

fn input_transform(line: String) -> InputEnt {
    line.chars().collect()
}

fn get_positions(input: &[InputEnt]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != '.' {
                positions
                    .entry(*c)
                    .and_modify(|v| v.push((x, y)))
                    .or_insert(vec![(x, y)]);
            }
        }
    }

    positions
}
