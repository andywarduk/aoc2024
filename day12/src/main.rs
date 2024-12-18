use std::{
    collections::{BTreeMap, VecDeque},
    error::Error,
};

use aoc::input::parse_input_vec;
use fxhash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(12, input_transform)?;
    let shapes = get_shapes(&input);

    // Run parts
    println!("Part 1: {}", part1(&shapes));
    println!("Part 2: {}", part2(&shapes));

    Ok(())
}

fn part1(shapes: &[Shape]) -> u64 {
    shapes.iter().map(|s| s.area * s.perimeter).sum()
}

fn part2(shapes: &[Shape]) -> u64 {
    shapes.iter().map(|s| s.area * s.sides).sum()
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Shape {
    area: u64,
    perimeter: u64,
    sides: u64,
}

fn get_shapes(input: &[InputEnt]) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut touched: FxHashSet<Coord> = FxHashSet::default();

    for (y, l) in input.iter().enumerate() {
        (0..l.len()).for_each(|x| {
            if !touched.contains(&(x, y)) {
                // Get shape topology
                let (squares, xbounds, ybounds) = shape_topology(input, x, y);

                // Add squares to touched list
                touched.extend(&squares);

                // Get perimeter and side count
                let (perimeter, sides) = perimeter_sides(&squares, &xbounds, &ybounds);

                // Add shape
                shapes.push(Shape {
                    area: squares.len() as u64,
                    perimeter,
                    sides,
                })
            }
        });
    }

    shapes
}

fn perimeter_sides(
    squares: &FxHashSet<Coord>,
    xbounds: &BTreeMap<usize, (usize, usize)>,
    ybounds: &BTreeMap<usize, (usize, usize)>,
) -> (u64, u64) {
    let mut perimeter = 0;
    let mut sides = 0;

    // Horizontal edges
    for (&y, &(xmin, xmax)) in xbounds {
        let mut lasttop = false;
        let mut lastbottom = false;

        for x in xmin..=xmax {
            if squares.contains(&(x, y)) {
                if y == 0 || !squares.contains(&(x, y - 1)) {
                    perimeter += 1;

                    if !lasttop {
                        sides += 1;
                        lasttop = true;
                    }
                } else {
                    lasttop = false;
                }

                if !squares.contains(&(x, y + 1)) {
                    perimeter += 1;

                    if !lastbottom {
                        sides += 1;
                        lastbottom = true;
                    }
                } else {
                    lastbottom = false;
                }
            } else {
                lasttop = false;
                lastbottom = false;
            }
        }
    }

    // Vertical edges
    for (&x, &(ymin, ymax)) in ybounds {
        let mut lastleft = false;
        let mut lastright = false;

        for y in ymin..=ymax {
            if squares.contains(&(x, y)) {
                if x == 0 || !squares.contains(&(x - 1, y)) {
                    perimeter += 1;

                    if !lastleft {
                        sides += 1;
                        lastleft = true;
                    }
                } else {
                    lastleft = false;
                }

                if !squares.contains(&(x + 1, y)) {
                    perimeter += 1;

                    if !lastright {
                        sides += 1;
                        lastright = true;
                    }
                } else {
                    lastright = false;
                }
            } else {
                lastleft = false;
                lastright = false;
            }
        }
    }

    (perimeter, sides)
}

type ShapeTopology = (
    FxHashSet<Coord>,
    BTreeMap<usize, (usize, usize)>,
    BTreeMap<usize, (usize, usize)>,
);

fn shape_topology(input: &[InputEnt], x: usize, y: usize) -> ShapeTopology {
    let mut squares = FxHashSet::default();
    let mut xbounds = BTreeMap::new();
    let mut ybounds = BTreeMap::new();

    let c = input[y][x];
    let mut work = VecDeque::new();

    // Add initial coordinate to squares set
    squares.insert((x, y));

    // Add first work item
    work.push_back((x, y));

    // Process work queue
    while let Some((x, y)) = work.pop_front() {
        // Update x and y bounds
        update_bounds(&mut xbounds, y, x);
        update_bounds(&mut ybounds, x, y);

        // Do flood fill step
        for (x1, y1) in flood_step(input, x, y, c) {
            if !squares.contains(&(x1, y1)) {
                squares.insert((x1, y1));
                work.push_back((x1, y1));
            }
        }
    }

    (squares, xbounds, ybounds)
}

fn update_bounds(bounds: &mut BTreeMap<usize, (usize, usize)>, key: usize, value: usize) {
    bounds
        .entry(key)
        .and_modify(|(min, max)| {
            *min = value.min(*min);
            *max = value.max(*max);
        })
        .or_insert((value, value));
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

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
AAAA
BBCD
BBCC
EEEC
";

    const EXAMPLE2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const EXAMPLE3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    const EXAMPLE4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const EXAMPLE5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let shapes = get_shapes(&input);
        assert_eq!(part1(&shapes), 140);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        let shapes = get_shapes(&input);
        assert_eq!(part1(&shapes), 772);
    }

    #[test]
    fn test3() {
        let input = parse_test_vec(EXAMPLE3, input_transform).unwrap();
        let shapes = get_shapes(&input);
        assert_eq!(part1(&shapes), 1930);
    }

    #[test]
    fn test4() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let shapes = get_shapes(&input);
        assert_eq!(part2(&shapes), 80);
    }

    #[test]
    fn test5() {
        let input = parse_test_vec(EXAMPLE4, input_transform).unwrap();
        let shapes = get_shapes(&input);
        assert_eq!(part2(&shapes), 236);
    }

    #[test]
    fn test6() {
        let input = parse_test_vec(EXAMPLE5, input_transform).unwrap();
        let shapes = get_shapes(&input);
        assert_eq!(part2(&shapes), 368);
    }

    #[test]
    fn test7() {
        let input = parse_test_vec(EXAMPLE3, input_transform).unwrap();
        let shapes = get_shapes(&input);
        assert_eq!(part2(&shapes), 1206);
    }
}
