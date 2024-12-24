use std::{collections::VecDeque, error::Error};

use aoc::input::read_input_file;
use fxhash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(24).unwrap();
    let circuit = parse_input(&input);

    // Run parts
    println!("Part 1: {}", part1(circuit.clone()));
    println!("Part 2: {}", part2(circuit));

    Ok(())
}

#[derive(Debug, Clone)]
struct Work {
    name: String,
    value: bool,
}

fn part1(mut circuit: Circuit) -> u64 {
    circuit.run();

    circuit.get_value('z')
}

fn part2(circuit: Circuit) -> u64 {
    0 // TODO
}

#[derive(Debug, Clone)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate {
    in_wires: [String; 2],
    out_wire: String,
    op: Op,
}

#[derive(Debug, Clone)]
struct Circuit {
    wires: FxHashMap<String, bool>,
    connections: FxHashMap<String, Vec<(usize, usize)>>,
    gates: Vec<Gate>,
}

impl Circuit {
    fn run(&mut self) {
        let mut work = VecDeque::new();

        for (name, &value) in &self.wires {
            work.push_back(Work {
                name: name.clone(),
                value,
            });
        }

        while let Some(work_ent) = work.pop_front() {
            if let Some(conns) = self.connections.get(&work_ent.name) {
                for (gate, inwire) in conns {
                    let gate = &mut self.gates[*gate];

                    // Got other wire?
                    let other = &gate.in_wires[1 - *inwire];

                    if let Some(value) = self.wires.get(other) {
                        // Got both values
                        let outval = match gate.op {
                            Op::And => work_ent.value & value,
                            Op::Or => work_ent.value | value,
                            Op::Xor => work_ent.value ^ value,
                        };

                        self.wires.insert(gate.out_wire.clone(), outval);

                        work.push_back(Work {
                            name: gate.out_wire.clone(),
                            value: outval,
                        });
                    }
                }
            }
        }
    }

    fn get_value(&self, prefix: char) -> u64 {
        let mut result = 0;

        for (name, &value) in &self.wires {
            if name.starts_with(prefix) && value {
                let bit = name.trim_start_matches('z').parse::<u8>().unwrap();
                result |= 2u64.pow(bit as u32);
            }
        }

        result
    }
}

// Input parsing

fn parse_input(input: &str) -> Circuit {
    let mut sections = input.split("\n\n");

    let s1 = sections.next().unwrap();
    let s2 = sections.next().unwrap();

    let mut wires = FxHashMap::default();
    let mut connections: FxHashMap<String, Vec<(usize, usize)>> = FxHashMap::default();

    s1.lines().for_each(|l| {
        let mut split = l.split(':');

        let name = split.next().unwrap().to_string();
        let value = split.next().unwrap().trim_start().parse::<u8>().unwrap();

        wires.insert(name, value == 1);
    });

    let gates = s2
        .lines()
        .enumerate()
        .map(|(num, l)| {
            let mut split = l.split_ascii_whitespace();

            let in1 = split.next().unwrap().to_string();
            let op = match split.next().unwrap() {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => panic!("Invalid op"),
            };
            let in2 = split.next().unwrap().to_string();
            split.next().unwrap();
            let out_wire = split.next().unwrap().to_string();

            let in_wires = [in1.clone(), in2.clone()];

            // Add wires
            // TODO Needed?
            // wires.insert(in1.clone(), None);
            // wires.insert(in2.clone(), None);
            // wires.insert(out.clone(), None);

            // Add connections
            connections.entry(in1).or_default().push((num, 0));
            connections.entry(in2).or_default().push((num, 1));

            Gate {
                in_wires,
                op,
                out_wire,
            }
        })
        .collect();

    Circuit {
        wires,
        connections,
        gates,
    }
}

#[cfg(test)]
mod tests;
