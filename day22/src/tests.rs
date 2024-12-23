use aoc::input::parse_test_vec;

use super::*;

const EXAMPLE1: &str = "\
15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254
";

#[test]
fn test1() {
    let mut secret = 123;

    let next = EXAMPLE1
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for n in next {
        hashstep(&mut secret);
        assert_eq!(secret, n);
    }
}

const EXAMPLE2: &str = "\
1: 8685429
10: 4700978
100: 15273692
2024: 8667524
";

#[test]
fn test2() {
    let sec_result = EXAMPLE2
        .lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();

            let secret = parts
                .next()
                .unwrap()
                .trim_end_matches(':')
                .parse::<u64>()
                .unwrap();
            let result = parts.next().unwrap().parse::<u64>().unwrap();

            (secret, result)
        })
        .collect::<Vec<_>>();

    let sum = sec_result
        .into_iter()
        .map(|(mut secret, result)| {
            for _ in 0..2000 {
                hashstep(&mut secret);
            }

            assert_eq!(secret, result);

            secret
        })
        .sum::<u64>();

    assert_eq!(sum, 37327623);
}

const EXAMPLE3: &str = "\
1
2
3
2024
";

#[test]
fn test3() {
    let input = parse_test_vec(EXAMPLE3, input_transform).unwrap();
    assert_eq!(part2(&input), 23);
}
