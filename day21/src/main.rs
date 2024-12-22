use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashMap;
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
    solve_chain(input, 2)
}

fn part2(input: &[InputEnt]) -> u64 {
    solve_chain(input, 25)
}

fn solve_chain(input: &[InputEnt], count: usize) -> u64 {
    let keypads = build_keypads(count);

    input
        .iter()
        .map(|keys| {
            let len = press_keys(&keypads, keys);

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

fn build_keypads(count: usize) -> Vec<KeyPad> {
    let mut dirkeypad = KeyPad::new(3, 2);
    // (0,0) empty
    dirkeypad.setkey((1, 0), Key::Action(Action::Up));
    dirkeypad.setkey((2, 0), Key::Action(Action::Activate));
    dirkeypad.setkey((0, 1), Key::Action(Action::Left));
    dirkeypad.setkey((1, 1), Key::Action(Action::Down));
    dirkeypad.setkey((2, 1), Key::Action(Action::Right));
    dirkeypad.build_routes(None);

    let mut numkeypad = KeyPad::new(3, 4);
    numkeypad.setkey((0, 0), Key::Char('7'));
    numkeypad.setkey((1, 0), Key::Char('8'));
    numkeypad.setkey((2, 0), Key::Char('9'));
    numkeypad.setkey((0, 1), Key::Char('4'));
    numkeypad.setkey((1, 1), Key::Char('5'));
    numkeypad.setkey((2, 1), Key::Char('6'));
    numkeypad.setkey((0, 2), Key::Char('1'));
    numkeypad.setkey((1, 2), Key::Char('2'));
    numkeypad.setkey((2, 2), Key::Char('3'));
    // (0,3) empty
    numkeypad.setkey((1, 3), Key::Char('0'));
    numkeypad.setkey((2, 3), Key::Action(Action::Activate));
    numkeypad.build_routes(Some(&dirkeypad));

    let mut keypads = vec![numkeypad];

    for _ in 0..=count {
        keypads.push(dirkeypad.clone());
    }

    keypads
}

fn press_keys(keypads: &[KeyPad], keys: &[Key]) -> u64 {
    let mut keys_cache = FxHashMap::default();

    press_keys_pad(keypads, 0, keys, &mut keys_cache)
}

fn press_keys_pad(
    keypads: &[KeyPad],
    pad: usize,
    keys: &[Key],
    keys_cache: &mut FxHashMap<(usize, Key, Key), u64>,
) -> u64 {
    if pad == keypads.len() - 1 {
        keys.len() as u64
    } else {
        keys.iter()
            .fold(
                (Key::Action(Action::Activate), 0),
                |(curkey, solutions), key| {
                    let key_sols = if let Some(key_sols) = keys_cache.get(&(pad, curkey, *key)) {
                        *key_sols
                    } else {
                        let key_sols = press_keys_key(keypads, pad, key, curkey, keys_cache);
                        keys_cache.insert((pad, curkey, *key), key_sols);
                        key_sols
                    };

                    (*key, solutions + key_sols)
                },
            )
            .1
    }
}

fn press_keys_key(
    keypads: &[KeyPad],
    pad: usize,
    key: &Key,
    curkey: Key,
    keys_cache: &mut FxHashMap<(usize, Key, Key), u64>,
) -> u64 {
    let paths = keypads[pad].routes(curkey, *key);

    let path = if paths.len() > 1 {
        let mut costs = paths
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let keys = convert_actions_to_keys(path);

                let solution = press_keys_pad(keypads, pad + 1, &keys, keys_cache);

                (solution, i)
            })
            .collect::<Vec<_>>();

        costs.sort_by(|a, b| a.0.cmp(&b.0));

        &paths[costs[0].1]
    } else {
        &paths[0]
    };

    let keys = convert_actions_to_keys(path);

    press_keys_pad(keypads, pad + 1, &keys, keys_cache)
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
        let keypads = build_keypads(0);

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
        let keypads = build_keypads(0);

        let keys = input_transform("029A".to_string());

        let min = press_keys(&keypads, &keys);

        assert_eq!(12, min);
    }

    #[test]
    fn test3() {
        let keypads = build_keypads(1);

        let keys = input_transform("029A".to_string());

        let min = press_keys(&keypads, &keys);

        assert_eq!(min, 28);
    }

    #[test]
    fn test4() {
        let keypads = build_keypads(2);

        let keys = input_transform("029A".to_string());

        let min = press_keys(&keypads, &keys);

        assert_eq!(min, 68);
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
