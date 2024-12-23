use std::error::Error;

use aoc::input::parse_input_vec;
use fxhash::FxHashMap;

mod keypad;
use keypad::{Action, Key, KeyPad, KeyPadBuilder};

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
    let mut keys_cache = KeysCache::default();

    // Iterate key sequences for numeric keypad
    let result = input
        .iter()
        .map(|keys| {
            // Calculate the fewest number of keys pressed on the human directional keypad
            let keypresses = press_keys(&keypads, 0, keys, &mut keys_cache);

            // Calculate the value of the numeric part of the typed code
            let code_value = keys
                .iter()
                .filter_map(|k| match k {
                    Key::Num(c) => Some(*c as u64),
                    _ => None,
                })
                .rev()
                .enumerate()
                .fold(0, |acc, (i, d)| acc + (d * 10u64.pow(i as u32)));

            // Multiply together
            keypresses * code_value
        })
        .sum();

    // Dump the key cache
    #[cfg(debug_assertions)]
    keys_cache.dump();

    result
}

fn build_keypads() -> (KeyPad, KeyPad) {
    // Create numeric keypad
    let numkeypad = KeyPadBuilder::new(3, 4)
        .setkey((0, 0), Key::Num(7))
        .setkey((1, 0), Key::Num(8))
        .setkey((2, 0), Key::Num(9))
        .setkey((0, 1), Key::Num(4))
        .setkey((1, 1), Key::Num(5))
        .setkey((2, 1), Key::Num(6))
        .setkey((0, 2), Key::Num(1))
        .setkey((1, 2), Key::Num(2))
        .setkey((2, 2), Key::Num(3))
        // (0,3) empty
        .setkey((1, 3), Key::Num(0))
        .setkey((2, 3), Key::Action(Action::Activate))
        .build();

    // Create directional keypad
    let dirkeypad = KeyPadBuilder::new(3, 2)
        // (0,0) empty
        .setkey((1, 0), Key::Action(Action::Up))
        .setkey((2, 0), Key::Action(Action::Activate))
        .setkey((0, 1), Key::Action(Action::Left))
        .setkey((1, 1), Key::Action(Action::Down))
        .setkey((2, 1), Key::Action(Action::Right))
        .build();

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

#[derive(Debug, Default)]
struct KeysCache {
    map: FxHashMap<(usize, Key, Key), u64>,
    #[cfg(debug_assertions)]
    lookup_count: FxHashMap<(usize, Key, Key), u64>,
}

impl KeysCache {
    fn add(&mut self, pad: usize, key_from: Key, key_to: Key, count: u64) {
        self.map.insert((pad, key_from, key_to), count);
    }

    fn lookup(&mut self, pad: usize, key_from: Key, key_to: Key) -> Option<&u64> {
        let result = self.map.get(&(pad, key_from, key_to));

        #[cfg(debug_assertions)]
        if result.is_some() {
            *(self
                .lookup_count
                .entry((pad, key_from, key_to))
                .or_insert(0)) += 1;
        }

        result
    }

    #[cfg(debug_assertions)]
    fn dump(&self) {
        let mut keys = self.map.keys().copied().collect::<Vec<_>>();
        keys.sort();

        println!("key cache ({} entries):", keys.len());

        for (pad, key_from, key_to) in keys {
            println!(
                "  pad {pad} from {key_from} to {key_to} : presses {}, lookups {}",
                *(self.map.get(&(pad, key_from, key_to)).unwrap()),
                *(self
                    .lookup_count
                    .get(&(pad, key_from, key_to))
                    .unwrap_or(&0))
            )
        }
    }
}

fn press_keys(keypads: &[KeyPad], pad: usize, keys: &[Key], keys_cache: &mut KeysCache) -> u64 {
    if pad == keypads.len() - 1 {
        // Last pad - just return the number of keys to be pressed
        keys.len() as u64
    } else {
        // Iterate the keys needing to be pressed (always starts at Activate)
        keys.iter()
            .fold(
                (Key::Action(Action::Activate), 0),
                |(curkey, total_keypresses), key| {
                    let keypresses = if curkey == *key {
                        // Just the action key needed as we're already in the right place
                        1
                    } else {
                        // Look up in the cache
                        if let Some(keypresses) = keys_cache.lookup(pad, curkey, *key) {
                            // Got cached entry
                            *keypresses
                        } else {
                            // Calculate number of key presses needed on the human directional keypad
                            let keypresses = press_keys_key(keypads, pad, key, curkey, keys_cache);

                            // Insert in to the cache
                            keys_cache.add(pad, curkey, *key, keypresses);

                            keypresses
                        }
                    };

                    // Accumulate passing next key and number of key presses so far
                    (*key, total_keypresses + keypresses)
                },
            )
            .1 // total_keypresses
    }
}

fn press_keys_key(
    keypads: &[KeyPad],
    pad: usize,
    key: &Key,
    curkey: Key,
    keys_cache: &mut KeysCache,
) -> u64 {
    // Get all valid shortest paths from key to key
    let paths = keypads[pad].routes(curkey, *key);

    let keys = if paths.len() > 1 {
        // More than one path - find shortest
        let shortest = paths
            .iter()
            .enumerate()
            .map(|(i, keys)| {
                // Calculate presses on the next pad recursively
                let keypresses = press_keys(keypads, pad + 1, keys, keys_cache);

                (keypresses, i)
            })
            .min()
            .map(|m| m.1)
            .unwrap();

        // Return the shortest
        &paths[shortest]
    } else {
        // Only one path
        &paths[0]
    };

    // Calculate presses on the next pad recursively
    press_keys(keypads, pad + 1, keys, keys_cache)
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
mod tests;
