use std::{collections::HashSet, error::Error};

use aoc::input::read_input_file;
use device::{Device, Reg};
use regex::Regex;

mod device;

fn main() -> Result<(), Box<dyn Error>> {
    let (rega, program) = parse_input();

    // Run parts
    println!("Part 1: {}", part1(rega, &program));
    println!("Part 2: {}", part2(&program));

    Ok(())
}

fn part1(rega: u64, program: &[u8]) -> String {
    let mut device = Device::new().reg(Reg::A, rega).program(program);

    device.run();

    let strvals = device
        .get_output()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

    strvals.join(",")
}

fn part2(program: &[u8]) -> u64 {
    // Find XOR ops
    let xors = program
        .chunks(2)
        .filter_map(|instr| {
            if instr[0] == 1 {
                // bxl
                Some(instr[1] as u64)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert!(xors.len() == 2);

    // Build valid answers
    let mut answers: HashSet<u64> = HashSet::default();
    answers.insert(0);

    for &num in program.iter().rev() {
        let mut new_answers = HashSet::default();

        for answer in answers {
            for i in 0..8 {
                let new_answer = (answer << 3) + i;
                let partial = (new_answer % 8) ^ xors[0];
                let out = (((partial ^ (new_answer >> partial)) ^ xors[1]) % 8) as u8;

                if out == num {
                    new_answers.insert(new_answer);
                }
            }
        }

        answers = new_answers;
    }

    // Get minimum answer
    let answer = answers.into_iter().min().unwrap();

    // Test the answer
    let mut device = Device::new().reg(Reg::A, answer).program(program);

    device.run();

    assert_eq!(program, *device.get_output());

    answer
}

// Input parsing

fn parse_input() -> (u64, Vec<u8>) {
    let input = read_input_file(17).unwrap();

    parse_input_string(&input)
}

fn parse_input_string(input: &str) -> (u64, Vec<u8>) {
    let prog_re = Regex::new(r"Program: ([\d,]*)").unwrap();

    let program = prog_re
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let rega_re = Regex::new(r"Register A: (\d*)").unwrap();

    let reg_a = rega_re
        .captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u64>()
        .unwrap();

    (reg_a, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut device = Device::new().debug(true).reg(Reg::C, 9).program(&[2, 6]);

        device.run();

        assert_eq!(device.get_reg(Reg::B), 1);
    }

    #[test]
    fn test2() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::A, 10)
            .program(&[5, 0, 5, 1, 5, 4]);

        device.run();

        assert_eq!(device.get_output(), &vec![0, 1, 2]);
    }

    #[test]
    fn test3() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::A, 2024)
            .program(&[0, 1, 5, 4, 3, 0]);

        device.run();

        assert_eq!(device.get_output(), &vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(device.get_reg(Reg::A), 0);
    }

    #[test]
    fn test4() {
        let mut device = Device::new().debug(true).reg(Reg::B, 29).program(&[1, 7]);

        device.run();

        assert_eq!(device.get_reg(Reg::B), 26);
    }

    #[test]
    fn test5() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::B, 2024)
            .reg(Reg::C, 43690)
            .program(&[4, 0]);

        device.run();

        assert_eq!(device.get_reg(Reg::B), 44354);
    }

    #[test]
    fn test6() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::A, 729)
            .program(&[0, 1, 5, 4, 3, 0]);

        device.run();

        assert_eq!(device.get_output(), &vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    const EXAMPLE1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn test7() {
        let (rega, program) = parse_input_string(EXAMPLE1);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(rega, &program));
    }
}
