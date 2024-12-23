use std::error::Error;

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
    let mut device = Device::new()
        .reg(Reg::A, rega)
        .program(program)
        .debug(cfg!(debug_assertions));

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
    let mut answer = 0;

    for &num in program.iter().rev() {
        for i in 0..8 {
            let next_answer = (answer << 3) + i;
            let partial = (next_answer % 8) ^ xors[0];
            let out = (((partial ^ (next_answer >> partial)) ^ xors[1]) % 8) as u8;

            if out == num {
                answer = next_answer;
                break;
            }
        }
    }

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
mod tests;
