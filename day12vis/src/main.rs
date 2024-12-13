use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    error::Error,
};

use aoc::{gif::Gif, input::parse_input_vec};
use hsl::HSL;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(12, input_transform)?;
    let shapes = get_shapes(&input);

    draw(&input, &shapes)?;

    Ok(())
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Shape {
    c: char,
    squares: HashSet<Coord>,
}

fn get_shapes(input: &[InputEnt]) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut touched: HashSet<Coord> = HashSet::new();

    for (y, l) in input.iter().enumerate() {
        for (x, &c) in l.iter().enumerate() {
            if !touched.contains(&(x, y)) {
                // Get shape topology
                let squares = shape_squares(input, x, y);

                // Add squares to touched list
                touched.extend(&squares);

                // Add shape
                shapes.push(Shape { c, squares })
            }
        }
    }

    shapes
}

const CELLSIZE: usize = 6;

fn draw(input: &[InputEnt], shapes: &[Shape]) -> Result<(), Box<dyn Error>> {
    let chars: BTreeSet<char> = input.iter().flat_map(|l| l.iter().copied()).collect();

    let mut palette = vec![[0, 0, 0], [255, 255, 255]];

    let pal_start = palette.len() as u8;

    for i in 0..chars.len() {
        let deg = (360.0 / chars.len() as f64) * i as f64;

        let hsl = HSL {
            h: deg,
            s: 1.0,
            l: 0.5,
        };

        let (r, g, b) = hsl.to_rgb();

        palette.push([r, g, b]);
    }

    let pal_dim = palette.len() as u8;

    for i in 0..chars.len() {
        let deg = (360.0 / chars.len() as f64) * i as f64;

        let hsl = HSL {
            h: deg,
            s: 0.1,
            l: 0.5,
        };

        let (r, g, b) = hsl.to_rgb();

        palette.push([r, g, b]);
    }

    let mut gif = Gif::new(
        "vis/day12.gif",
        &palette,
        (input[0].len() * CELLSIZE) as u16,
        (input.len() * CELLSIZE) as u16,
        1,
        1,
    )?;

    draw_frame(&mut gif, shapes, |c, fence| {
        if fence {
            0
        } else {
            chars.iter().position(|&cp| cp == c).unwrap() as u8 + pal_start
        }
    })?;

    draw_frame(&mut gif, shapes, |c, fence| {
        let pos = chars.iter().position(|&cp| cp == c).unwrap() as u8;

        if fence {
            pos + pal_start
        } else {
            pos + pal_dim
        }
    })?;

    Ok(())
}

fn draw_frame<F>(gif: &mut Gif, shapes: &[Shape], colour: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(char, bool) -> u8,
{
    let mut frame = gif.empty_frame();

    for shape in shapes {
        draw_shape(&mut frame, shape, &colour);
    }

    gif.draw_frame(frame, 300)?;

    Ok(())
}

fn draw_shape<F>(frame: &mut [Vec<u8>], shape: &Shape, colour: F)
where
    F: Fn(char, bool) -> u8,
{
    let drawhoriz = |frame: &mut [Vec<u8>], y: usize, x1: usize, x2: usize, col: u8| {
        for x in x1..=x2 {
            frame[y][x] = col;
        }
    };

    for &(x, y) in shape.squares.iter() {
        let mut sx = 1;
        let mut ex = CELLSIZE - 2;
        let mut sy = 1;
        let mut ey = CELLSIZE - 2;

        if x > 0 && shape.squares.contains(&(x - 1, y)) {
            sx = 0;
        }

        if shape.squares.contains(&(x + 1, y)) {
            ex = CELLSIZE - 1;
        }

        if y > 0 && shape.squares.contains(&(x, y - 1)) {
            sy = 0;
        }

        if shape.squares.contains(&(x, y + 1)) {
            ey = CELLSIZE - 1;
        }

        let gx = x * CELLSIZE;
        let gy = y * CELLSIZE;

        let fence_colour = colour(shape.c, true);
        let inner_colour = colour(shape.c, false);

        for i in 0..sy {
            drawhoriz(frame, gy + i, gx, gx + CELLSIZE - 1, fence_colour);
        }

        for i in sy..=ey {
            drawhoriz(frame, gy + i, gx + sx, gx + ex, inner_colour);

            drawhoriz(frame, gy + i, gx, gx + sx - 1, fence_colour);
            drawhoriz(frame, gy + i, gx + ex + 1, gx + CELLSIZE - 1, fence_colour);
        }

        for i in (ey + 1)..CELLSIZE {
            drawhoriz(frame, gy + i, gx, gx + CELLSIZE - 1, fence_colour);
        }
    }
}

fn shape_squares(input: &[InputEnt], x: usize, y: usize) -> HashSet<Coord> {
    let mut squares = HashSet::new();

    let c = input[y][x];
    let mut work = VecDeque::new();

    // Add initial coordinate to squares set
    squares.insert((x, y));

    // Add first work item
    work.push_back((x, y));

    // Process work queue
    while let Some((x, y)) = work.pop_front() {
        // Do flood fill step
        for (x1, y1) in flood_step(input, x, y, c) {
            if !squares.contains(&(x1, y1)) {
                squares.insert((x1, y1));
                work.push_back((x1, y1));
            }
        }
    }

    squares
}

const DIRS: [[isize; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

fn flood_step(input: &[InputEnt], x: usize, y: usize, c: char) -> impl Iterator<Item = Coord> {
    DIRS.iter().filter_map(move |&[dx, dy]| {
        match x.checked_add_signed(dx) {
            Some(nx) if nx < input[0].len() => match y.checked_add_signed(dy) {
                Some(ny) if ny < input.len() => {
                    if input[ny][nx] == c {
                        return Some((nx, ny));
                    }
                }
                _ => (),
            },
            _ => (),
        }

        None
    })
}

// Input parsing

type InputEnt = Vec<char>;

fn input_transform(line: String) -> InputEnt {
    line.chars().collect()
}
