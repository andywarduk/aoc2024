use std::error::Error;

use aoc::input::parse_input_vec;
use keypad::{Action, Key, KeyPad};

mod keypad;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(21, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    let keypads = build_keypads();

    input
        .iter()
        .map(|keys| {
            let solutions = press_keys(&keypads, keys);

            let len = solutions[0][3].len() as u64;

            let val = keys
                .iter()
                .filter_map(|k| match k {
                    Key::Char(c) if c.is_ascii_digit() => Some(*c as u8 - b'0'),
                    _ => None,
                })
                .rev()
                .enumerate()
                .fold(0, |acc, (i, d)| acc + (d as u64 * 10u64.pow(i as u32)));

            println!("{} * {}", len, val);
            len * val
        })
        .sum()
}

fn part2(input: &[InputEnt]) -> u64 {
    0 // TODO
}

fn build_keypads() -> Vec<KeyPad> {
    let mut keypad1 = KeyPad::new(3, 4);
    keypad1.setkey((0, 0), Key::Char('7'));
    keypad1.setkey((1, 0), Key::Char('8'));
    keypad1.setkey((2, 0), Key::Char('9'));
    keypad1.setkey((0, 1), Key::Char('4'));
    keypad1.setkey((1, 1), Key::Char('5'));
    keypad1.setkey((2, 1), Key::Char('6'));
    keypad1.setkey((0, 2), Key::Char('1'));
    keypad1.setkey((1, 2), Key::Char('2'));
    keypad1.setkey((2, 2), Key::Char('3'));
    // (0,3) empty
    keypad1.setkey((1, 3), Key::Char('0'));
    keypad1.setkey((2, 3), Key::Action(Action::Activate));
    keypad1.build_routes();

    let mut keypad2 = KeyPad::new(3, 2);
    // (0,0) empty
    keypad2.setkey((1, 0), Key::Action(Action::Up));
    keypad2.setkey((2, 0), Key::Action(Action::Activate));
    keypad2.setkey((0, 1), Key::Action(Action::Left));
    keypad2.setkey((1, 1), Key::Action(Action::Down));
    keypad2.setkey((2, 1), Key::Action(Action::Right));
    keypad2.build_routes();

    let keypad3 = keypad2.clone();
    let keypad4 = keypad2.clone();

    vec![keypad1, keypad2, keypad3, keypad4]
}

#[derive(Debug, Clone)]
struct Solution {
    curkey: Vec<Key>,
    actions: Vec<Vec<Action>>,
}

fn press_keys(keypads: &[KeyPad], keys: &[Key]) -> Vec<Vec<Vec<Action>>> {
    let solution = Solution {
        curkey: vec![Key::Action(Action::Activate); keypads.len()],
        actions: vec![vec![]; keypads.len()],
    };

    let solutions = press_keys_pad(keypads, 0, keys, solution);

    let min = solutions
        .iter()
        .map(|s| s.actions[keypads.len() - 1].len())
        .min()
        .unwrap();

    solutions
        .into_iter()
        .filter_map(|s| {
            if s.actions[keypads.len() - 1].len() == min {
                Some(s.actions)
            } else {
                None
            }
        })
        .collect()
}

fn press_keys_pad(
    keypads: &[KeyPad],
    pad: usize,
    keys: &[Key],
    solution: Solution,
) -> Vec<Solution> {
    let mut solutions = vec![solution];

    for key in keys {
        let mut new_solutions = Vec::new();

        for solution in solutions {
            new_solutions.extend(press_keys_key(keypads, pad, key, solution));
        }

        solutions = new_solutions;
    }

    solutions
}

fn press_keys_key(
    keypads: &[KeyPad],
    pad: usize,
    key: &Key,
    mut solution: Solution,
) -> Vec<Solution> {
    let mut new_solutions = Vec::new();

    // println!(
    //     "Pressing key {key:?} on pad {pad}. Currently on {:?}",
    //     solution.curkey[pad]
    // );

    if pad + 1 == keypads.len() {
        solution.curkey[pad] = *key;

        new_solutions.push(solution);
    } else {
        let paths = keypads[pad].routes(solution.curkey[pad], *key);

        for path in paths.iter() {
            let keys = convert_actions_to_keys(path);

            // Recurse
            let mut new_solution = solution.clone();

            new_solution.curkey[pad] = *key;
            new_solution.actions[pad + 1].extend(path);

            new_solutions.extend(press_keys_pad(keypads, pad + 1, &keys, new_solution));
        }
    }

    new_solutions
}

fn convert_actions_to_keys(actions: &[Action]) -> Vec<Key> {
    actions.iter().map(|a| Key::Action(*a)).collect()
}

// Input parsing

type InputEnt = Vec<Key>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                Key::Char(c)
            } else {
                Key::Action(match c {
                    'A' => Action::Activate,
                    _ => panic!("Invalid action: {}", c),
                })
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    #[test]
    fn test1() {
        let keypads = build_keypads();

        assert_eq!(
            keypads[0].routes(Key::Action(Action::Activate), Key::Char('0')),
            &vec![vec![Action::Left, Action::Activate]]
        );
        assert_eq!(keypads[0].routes(Key::Char('0'), Key::Char('2')), &vec![
            vec![Action::Up, Action::Activate]
        ]);
        assert_eq!(keypads[0].routes(Key::Char('2'), Key::Char('9')), &vec![
            vec![Action::Right, Action::Up, Action::Up, Action::Activate],
            vec![Action::Up, Action::Up, Action::Right, Action::Activate],
        ]);
        assert_eq!(
            keypads[0].routes(Key::Char('9'), Key::Action(Action::Activate)),
            &vec![vec![
                Action::Down,
                Action::Down,
                Action::Down,
                Action::Activate
            ]]
        );
    }

    #[test]
    fn test2() {
        let keypads = build_keypads();

        let keys = input_transform("029A".to_string());

        let solutions = press_keys(&keypads[0..2], &keys);

        println!("sols: {:?}", solutions);

        use Action::*;

        assert_eq!(solutions, vec![
            vec![vec![], vec![
                Left, Activate, Up, Activate, Right, Up, Up, Activate, Down, Down, Down, Activate
            ]],
            vec![vec![], vec![
                Left, Activate, Up, Activate, Up, Up, Right, Activate, Down, Down, Down, Activate
            ]]
        ]);
    }

    #[test]
    fn test3() {
        let keypads = build_keypads();

        let keys = input_transform("029A".to_string());

        let solutions = press_keys(&keypads[0..3], &keys);

        println!("sol[0]: {:?}", solutions[0]);

        assert_eq!(solutions[0][2].len(), 28);
    }

    #[test]
    fn test4() {
        let keypads = build_keypads();

        let keys = input_transform("029A".to_string());

        let solutions = press_keys(&keypads, &keys);

        println!("sol[0]: {:?}", solutions[0]);

        assert_eq!(solutions[0][3].len(), 68);
    }

    const EXAMPLE1: &str = "\
029A
980A
179A
456A
379A
";

    #[test]
    fn test5() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 126384);
    }
}
