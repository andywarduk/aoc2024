use std::{collections::HashSet, error::Error};

use device::{Device, Reg};

mod device;

fn main() -> Result<(), Box<dyn Error>> {
    // Run parts
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());

    Ok(())
}

const PROGRAM: [u8; 16] = [2, 4, 1, 3, 7, 5, 0, 3, 1, 4, 4, 7, 5, 5, 3, 0];

fn part1() -> String {
    let mut device = Device::new()
        .reg(Reg::A, 50230824)
        .program(PROGRAM.to_vec());

    device.run();

    let strvals = device
        .get_output()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

    strvals.join(",")
}

fn part2() -> u64 {
    let mut answers: HashSet<u64> = HashSet::default();
    answers.insert(0);

    for &num in PROGRAM.iter().rev() {
        let mut new_answers = HashSet::default();

        for curr in answers {
            for i in 0..8 {
                let new = (curr << 3) + i;
                let partial = (new % 8) ^ 3;
                let out = (((partial ^ (new >> partial)) ^ 4) % 8) as u8;

                if out == num {
                    new_answers.insert(new);
                }
            }
        }
        answers = new_answers;
    }

    let answer = *answers.iter().min().unwrap();

    let mut device = Device::new().reg(Reg::A, answer).program(PROGRAM.to_vec());

    device.run();

    assert_eq!(PROGRAM, **device.get_output());

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut device = Device::new().debug(true).reg(Reg::C, 9).program(vec![2, 6]);

        device.run();

        assert_eq!(device.get_reg(Reg::B), 1);
    }

    #[test]
    fn test2() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::A, 10)
            .program(vec![5, 0, 5, 1, 5, 4]);

        device.run();

        assert_eq!(device.get_output(), &vec![0, 1, 2]);
    }

    #[test]
    fn test3() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::A, 2024)
            .program(vec![0, 1, 5, 4, 3, 0]);

        device.run();

        assert_eq!(device.get_output(), &vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(device.get_reg(Reg::A), 0);
    }

    #[test]
    fn test4() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::B, 29)
            .program(vec![1, 7]);

        device.run();

        assert_eq!(device.get_reg(Reg::B), 26);
    }

    #[test]
    fn test5() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::B, 2024)
            .reg(Reg::C, 43690)
            .program(vec![4, 0]);

        device.run();

        assert_eq!(device.get_reg(Reg::B), 44354);
    }

    #[test]
    fn test6() {
        let mut device = Device::new()
            .debug(true)
            .reg(Reg::A, 729)
            .program(vec![0, 1, 5, 4, 3, 0]);

        device.run();

        assert_eq!(device.get_output(), &vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
