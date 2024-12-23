use super::*;

#[test]
fn test1() {
    let mut device = Device::new().debug(true).reg(Reg::C, 9).program(&[2, 6]); // b = c % 8

    device.run();

    assert_eq!(device.get_reg(Reg::B), 1);
}

#[test]
fn test2() {
    let mut device = Device::new()
        .debug(true)
        .reg(Reg::A, 10)
        .program(&[5, 0, 5, 1, 5, 4]); // out 0 % 8; out 1 % 8; out a % 8

    device.run();

    assert_eq!(device.get_output(), &vec![0, 1, 2]);
}

#[test]
fn test3() {
    let mut device = Device::new()
        .debug(true)
        .reg(Reg::A, 2024)
        .program(&[0, 1, 5, 4, 3, 0]); // a /= 2; out a % 8; if a <> 0 loop

    device.run();

    assert_eq!(device.get_output(), &vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    assert_eq!(device.get_reg(Reg::A), 0);
}

#[test]
fn test4() {
    let mut device = Device::new().debug(true).reg(Reg::B, 29).program(&[1, 7]); // b ^= 7

    device.run();

    assert_eq!(device.get_reg(Reg::B), 26);
}

#[test]
fn test5() {
    let mut device = Device::new()
        .debug(true)
        .reg(Reg::B, 2024)
        .reg(Reg::C, 43690)
        .program(&[4, 0]); // b ^= c

    device.run();

    assert_eq!(device.get_reg(Reg::B), 44354);
}

#[test]
fn test6() {
    let mut device = Device::new()
        .debug(true)
        .reg(Reg::A, 729)
        .program(&[0, 1, 5, 4, 3, 0]); // a /= 2; out a % 8; if a <> 0 loop

    device.run();

    assert_eq!(device.get_output(), &vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
}

// a /= 2; out a % 8; if a <> 0 loop
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

// a /= 8; out a % 8; if a <> 0 loop
const EXAMPLE3: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

#[test]
fn test8() {
    let (_, program) = parse_input_string(EXAMPLE3);

    let mut device = Device::new()
        .debug(true)
        .reg(Reg::A, 0o345300 /* 117440 */)
        .program(&program);

    device.run();

    assert_eq!(device.get_output(), &program);
}
