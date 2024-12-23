use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashMap;
use keypad::{Action, Key, KeyPad};

mod keypad;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(21, input_transform)?;

    let (numkeypad, dirkeypad) = build_keypads();

    // Run parts
    println!("Part 1: {}", part1(&input, &numkeypad, &dirkeypad));
    println!("Part 2: {}", part2(&input, &numkeypad, &dirkeypad));

    Ok(())
}

fn part1(input: &[InputEnt], numkeypad: &KeyPad, dirkeypad: &KeyPad) -> u64 {
    // Solve chain of 1 robot numeric keypad, 2 intermediate robot directional keypads and 1 human directional keypad
    solve_chain(input, 2, numkeypad, dirkeypad)
}

fn part2(input: &[InputEnt], numkeypad: &KeyPad, dirkeypad: &KeyPad) -> u64 {
    // Solve chain of 1 robot numeric keypad, 25 intermediate robot directional keypads and 1 human directional keypad
    solve_chain(input, 25, numkeypad, dirkeypad)
}

fn solve_chain(input: &[InputEnt], count: usize, numkeypad: &KeyPad, dirkeypad: &KeyPad) -> u64 {
    // Buld keypad chain
    let keypads = build_keypad_chain(numkeypad, dirkeypad, count);

    // Create a cache of (pad, key from, key to) to key sequence length on the human directional keypad
    let mut keys_cache = FxHashMap::default();

    // Iterate key sequences for numeric keypad
    input
        .iter()
        .map(|keys| {
            // Calculate the fewest number of keys pressed on the human directional keypad
            let len = press_keys(&keypads, keys, &mut keys_cache);

            // Calculate the value of the numeric part of the typed code
            let val = keys
                .iter()
                .filter_map(|k| match k {
                    Key::Num(c) => Some(*c as u64),
                    _ => None,
                })
                .rev()
                .enumerate()
                .fold(0, |acc, (i, d)| acc + (d * 10u64.pow(i as u32)));

            // Multiply together
            len * val
        })
        .sum()
}

fn build_keypads() -> (KeyPad, KeyPad) {
    // Create directional keypad
    let mut dirkeypad = KeyPad::new(3, 2);
    // (0,0) empty
    dirkeypad.setkey((1, 0), Key::Action(Action::Up));
    dirkeypad.setkey((2, 0), Key::Action(Action::Activate));
    dirkeypad.setkey((0, 1), Key::Action(Action::Left));
    dirkeypad.setkey((1, 1), Key::Action(Action::Down));
    dirkeypad.setkey((2, 1), Key::Action(Action::Right));
    dirkeypad.build_routes(None);

    // Create numeric keypad
    let mut numkeypad = KeyPad::new(3, 4);
    numkeypad.setkey((0, 0), Key::Num(7));
    numkeypad.setkey((1, 0), Key::Num(8));
    numkeypad.setkey((2, 0), Key::Num(9));
    numkeypad.setkey((0, 1), Key::Num(4));
    numkeypad.setkey((1, 1), Key::Num(5));
    numkeypad.setkey((2, 1), Key::Num(6));
    numkeypad.setkey((0, 2), Key::Num(1));
    numkeypad.setkey((1, 2), Key::Num(2));
    numkeypad.setkey((2, 2), Key::Num(3));
    // (0,3) empty
    numkeypad.setkey((1, 3), Key::Num(0));
    numkeypad.setkey((2, 3), Key::Action(Action::Activate));
    numkeypad.build_routes(Some(&dirkeypad));

    (numkeypad, dirkeypad)
}

fn build_keypad_chain(numkeypad: &KeyPad, dirkeypad: &KeyPad, count: usize) -> Vec<KeyPad> {
    // Create vector of keypads starting with the numeric keypad
    let mut keypads = vec![numkeypad.clone()];

    // Add intermediate keypads and human controlled keypad
    for _ in 0..=count {
        keypads.push(dirkeypad.clone());
    }

    keypads
}

fn press_keys(
    keypads: &[KeyPad],
    keys: &[Key],
    keys_cache: &mut FxHashMap<(usize, Key, Key), u64>,
) -> u64 {
    // Process the key sequence
    press_keys_pad(keypads, 0, keys, keys_cache)
}

fn press_keys_pad(
    keypads: &[KeyPad],
    pad: usize,
    keys: &[Key],
    keys_cache: &mut FxHashMap<(usize, Key, Key), u64>,
) -> u64 {
    if pad == keypads.len() - 1 {
        // Last pad - just return the number of keys pressed
        keys.len() as u64
    } else {
        // Iterate the keys needing to be pressed (always starts at Activate)
        keys.iter()
            .fold(
                (Key::Action(Action::Activate), 0),
                |(curkey, solutions), key| {
                    // Look up in the cache
                    let key_sols = if let Some(key_sols) = keys_cache.get(&(pad, curkey, *key)) {
                        // Got cached entry
                        *key_sols
                    } else {
                        // Calculate number of key presses needed on the human directional keypad
                        let key_sols = press_keys_key(keypads, pad, key, curkey, keys_cache);

                        // Insert in to the cache
                        keys_cache.insert((pad, curkey, *key), key_sols);

                        key_sols
                    };

                    // Accumulate passing next key and number of key presses so far
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
    // Get all valid shortest paths from key to key
    let paths = keypads[pad].routes(curkey, *key);

    let path = if paths.len() > 1 {
        // More than one path - recurse
        let mut presses = paths
            .iter()
            .enumerate()
            .map(|(i, path)| {
                // Convert actions to keys
                let keys = convert_actions_to_keys(path);

                // Calculate presses on the next pad recursively
                let presses = press_keys_pad(keypads, pad + 1, &keys, keys_cache);

                (presses, i)
            })
            .collect::<Vec<_>>();

        // Sort the array by number of presses
        presses.sort_by(|a, b| a.0.cmp(&b.0));

        // Return the shortest
        &paths[presses[0].1]
    } else {
        // Only one path
        &paths[0]
    };

    // Convert actions to keys
    let keys = convert_actions_to_keys(path);

    // Calculate presses on the next pad recursively
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
                Key::Num(c as u8 - b'0')
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
        let (numkeypad, dirkeypad) = build_keypads();
        let keypads = build_keypad_chain(&numkeypad, &dirkeypad, 0);

        assert_eq!(
            keypads[0].routes(Key::Action(Action::Activate), Key::Num(0)),
            &vec![vec![Action::Left, Action::Activate]]
        );
        assert_eq!(keypads[0].routes(Key::Num(0), Key::Num(2)), &vec![vec![
            Action::Up,
            Action::Activate
        ]]);
        assert_eq!(keypads[0].routes(Key::Num(2), Key::Num(9)), &vec![
            vec![Action::Up, Action::Up, Action::Right, Action::Activate],
            vec![Action::Right, Action::Up, Action::Up, Action::Activate],
        ]);
        assert_eq!(
            keypads[0].routes(Key::Num(9), Key::Action(Action::Activate)),
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
        let (numkeypad, dirkeypad) = build_keypads();
        let keypads = build_keypad_chain(&numkeypad, &dirkeypad, 0);
        let mut keys_cache = FxHashMap::default();

        let keys = input_transform("029A".to_string());

        let min = press_keys(&keypads, &keys, &mut keys_cache);

        assert_eq!(12, min);
    }

    #[test]
    fn test3() {
        let (numkeypad, dirkeypad) = build_keypads();
        let keypads = build_keypad_chain(&numkeypad, &dirkeypad, 1);
        let mut keys_cache = FxHashMap::default();

        let keys = input_transform("029A".to_string());

        let min = press_keys(&keypads, &keys, &mut keys_cache);

        assert_eq!(min, 28);
    }

    #[test]
    fn test4() {
        let (numkeypad, dirkeypad) = build_keypads();
        let keypads = build_keypad_chain(&numkeypad, &dirkeypad, 2);
        let mut keys_cache = FxHashMap::default();

        let keys = input_transform("029A".to_string());

        let min = press_keys(&keypads, &keys, &mut keys_cache);

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

        let (numkeypad, dirkeypad) = build_keypads();

        assert_eq!(part1(&input, &numkeypad, &dirkeypad), 126384);
    }
}
