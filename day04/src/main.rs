use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(4, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

const DIRECTIONS: [(i8, i8); 8] = [
    (1, 0),   // E
    (1, 1),   // SE
    (0, 1),   // S
    (-1, 1),  // SW
    (-1, 0),  // W
    (-1, -1), // NW
    (0, -1),  // N
    (1, -1),  // NE
];

fn part1(input: &[InputEnt]) -> u64 {
    let mut matches = 0;

    let check_word = |mut x: isize, mut y: isize, dx: isize, dy: isize| -> bool {
        let maxx = input[0].len() as isize;
        let maxy = input.len() as isize;

        // Check for MAS in the given direction
        for i in 1..4 {
            // Change X
            x += dx;

            // Bounds check
            if x < 0 || x >= maxx {
                return false;
            }

            // Change Y
            y += dy;

            // Bounds check
            if y < 0 || y >= maxy {
                return false;
            }

            // Check the board
            if input[y as usize][x as usize] != i {
                return false;
            }
        }

        true
    };

    // Loop each baord position
    for (y, r) in input.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            // Got an X?
            if *c == 0 {
                // Search in all directions
                for (dx, dy) in DIRECTIONS {
                    // Check for th word in this direction
                    if check_word(x as isize, y as isize, dx as isize, dy as isize) {
                        // Word found
                        matches += 1;
                    }
                }
            }
        }
    }

    matches
}

fn part2(input: &[InputEnt]) -> u64 {
    let mut matches = 0;

    // Function to check we have M and S or S and M in the board contents provided
    let check = |a, b| matches!((a, b), (1, 3) | (3, 1));

    // Loop the board skipping the first and last rows and columns
    for (y, r) in input.iter().enumerate().rev().skip(1).rev().skip(1) {
        for (x, c) in r.iter().enumerate().rev().skip(1).rev().skip(1) {
            // Check for A and call the check function with contents of the diagonals
            if *c == 2
                && check(input[y - 1][x - 1], input[y + 1][x + 1])
                && check(input[y - 1][x + 1], input[y + 1][x - 1])
            {
                // Found
                matches += 1;
            }
        }
    }

    matches
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: String) -> InputEnt {
    // Convert board chars to word letter index
    line.chars()
        .map(|c| match c {
            'X' => 0,
            'M' => 1,
            'A' => 2,
            'S' => 3,
            _ => panic!("Invalid char {c}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 18);
        assert_eq!(part2(&input), 9);
    }
}
