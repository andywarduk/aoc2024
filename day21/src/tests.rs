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
    let mut keys_cache = KeysCache::default();

    let keys = input_transform("029A".to_string());

    let min = press_keys(&keypads, &keys, &mut keys_cache);

    assert_eq!(12, min);
}

#[test]
fn test3() {
    let (numkeypad, dirkeypad) = build_keypads();
    let keypads = build_keypad_chain(&numkeypad, &dirkeypad, 1);
    let mut keys_cache = KeysCache::default();

    let keys = input_transform("029A".to_string());

    let min = press_keys(&keypads, &keys, &mut keys_cache);

    assert_eq!(min, 28);
}

#[test]
fn test4() {
    let (numkeypad, dirkeypad) = build_keypads();
    let keypads = build_keypad_chain(&numkeypad, &dirkeypad, 2);
    let mut keys_cache = KeysCache::default();

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
