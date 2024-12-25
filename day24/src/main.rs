use std::error::Error;

use aoc::input::read_input_file;
use fxhash::{FxHashMap, FxHashSet};

mod circuit;
use circuit::{Circuit, Gate, Op};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(24).unwrap();
    let mut circuit = parse_input(&input);

    // Run parts
    println!("Part 1: {}", part1(&mut circuit));
    println!("Part 2: {}", part2(&mut circuit));

    Ok(())
}

fn part1(circuit: &mut Circuit) -> u64 {
    circuit.run();

    circuit.get_value('z')
}

fn part2(circuit: &mut Circuit) -> String {
    // Half adder:
    //
    // X0--------o-----XOR
    //           |     XOR---------Rn
    // Y0-----o--------XOR
    //        |  |
    //        |  +---- AND
    //        |        AND---------Cn
    //        +--------AND

    // Full adder:
    //
    // Xn--------o-----XOR
    //           |     XOR----Hn--o---------XOR
    // Yn-----o--------XOR        |         XOR----------------------Rn
    //        |  |                |   +-----XOR
    //        |  +---- AND        |   |
    //        |        AND----+   +-- | ----AND
    //        +--------AND    |       |     AND--CAn--+
    //                        |       o-----AND       |
    //                        |       |               |
    // Cn-1---------------------------+               +---OR
    //                        |                           OR---------Cn
    //                        +-----------CBn-------------OR

    // Run the circuit
    circuit.run();

    // Count the number of bits in x input
    let bits = circuit.count_bits('x');

    // Initialise swaps vector
    let mut swaps = Vec::new();

    // Loop each bit
    for bit in 0..bits {
        // Follow the bit logic and get any errors
        let errors = follow_bit(circuit, bits, bit);

        // Got any errors?
        if !errors.is_empty() {
            // Yes - should be length 2
            assert_eq!(errors.len(), 2);

            // Swap the wires
            circuit.swap_wire(&errors[0], &errors[1]);

            // Record swapped wires
            swaps.extend(errors);
        }
    }

    #[cfg(debug_assertions)]
    {
        // Check the circuit
        let incorrect = check_circuit(circuit);

        if !incorrect.is_empty() {
            panic!("The following bits are incorrect: {incorrect:?}");
        }
    }

    // Sort swaps
    swaps.sort();

    // Return swaps joined by ,
    swaps.join(",")
}

#[derive(Debug, Default)]
struct FollowContext {
    bits: usize,
    bit: usize,
    stack: Vec<String>,
    errors: FxHashSet<String>,
}

fn follow_bit(circuit: &mut Circuit, bits: usize, bit: usize) -> Vec<String> {
    // Build input names
    let inx = Circuit::inoutname('x', bit);
    let iny = Circuit::inoutname('y', bit);

    // Build context
    let mut context = FollowContext {
        bits,
        bit,
        stack: vec![format!("Bit {bit}")],
        errors: Default::default(),
    };

    // Follow input wires
    follow_bit_input(circuit, &mut context, &inx, &iny);

    // Convert error hashset to vector
    context.errors.into_iter().collect::<Vec<_>>()
}

fn follow_bit_input(circuit: &mut Circuit, context: &mut FollowContext, inx: &str, iny: &str) {
    // Find gates with x and y inputs for this bit
    let gates = circuit.find_gates_with_inconn2(inx, iny);

    // Should be 2 - and XOR and an AND
    let mut got_xor = false;
    let mut got_and = false;

    for g in gates {
        match circuit.gate(g).op() {
            Op::Xor => {
                // Found XOR gate
                if context.bit == 0 {
                    // Half adder
                    follow_result(context, circuit.gate(g).out_wire())
                } else {
                    // Full adder
                    follow_half(circuit, context, circuit.gate(g).out_wire())
                }

                got_xor = true;
            }
            Op::And => {
                // Got AND gate
                if context.bit == 0 {
                    // Half adder
                    follow_carry(circuit, context, circuit.gate(g).out_wire())
                } else {
                    // Full adder
                    follow_carryb(circuit, context, circuit.gate(g).out_wire())
                }

                got_and = true;
            }
            _ => {
                // Unexpected gate
                follow_err(
                    context,
                    format!("Found {} gate with {inx} and {iny}", circuit.gate(g).op()),
                );
            }
        }
    }

    if !got_xor {
        // XOR not found
        follow_err(context, format!("No XOR gate with {inx} and {iny} found"));
    }

    if !got_and {
        // AND not found
        follow_err(context, format!("No AND gate with {inx} and {iny} found"));
    }
}

fn follow_result(context: &mut FollowContext, wire: String) {
    context.stack.push(format!("Result ({wire})"));

    // Build expected output wire name
    let expected = Circuit::inoutname('z', context.bit);

    // Check
    if wire != expected {
        // Not correct
        follow_err(context, format!("Result should go to {expected}"));

        context.errors.insert(wire.clone());
        context.errors.insert(expected);
    }

    context.stack.pop();
}

fn follow_half(circuit: &mut Circuit, context: &mut FollowContext, wire: String) {
    context.stack.push(format!("Half add ({wire})"));

    follow_half_or_carry(circuit, context, wire, "half add", true);

    context.stack.pop();
}

fn follow_carry(circuit: &mut Circuit, context: &mut FollowContext, wire: String) {
    context.stack.push(format!("Carry ({wire})"));

    let next_bit = context.bit + 1;

    // The last bit's carry goes to an output
    if next_bit == context.bits {
        let expected = Circuit::inoutname('z', next_bit);

        if wire != expected {
            follow_err(context, format!("Last carry should go to {expected}"));
        }
    } else {
        follow_half_or_carry(circuit, context, wire, "carry", false)
    }

    context.stack.pop();
}

fn follow_half_or_carry(
    circuit: &mut Circuit,
    context: &mut FollowContext,
    wire: String,
    desc: &str,
    follow: bool,
) {
    // Find gates with thsi wire as an input
    let gates = circuit.find_gates_with_inconn(&wire);

    // Should go to XOR and AND
    let mut got_xor = false;
    let mut got_and = false;

    for g in gates {
        match circuit.gate(g).op() {
            Op::Xor => {
                // Found XOR
                if follow {
                    follow_result(context, circuit.gate(g).out_wire());
                }

                got_xor = true;
            }
            Op::And => {
                // Found AND
                if follow {
                    follow_carrya(circuit, context, circuit.gate(g).out_wire());
                }

                got_and = true;
            }
            _ => {
                // Unexpected gate
                follow_err(
                    context,
                    format!(
                        "Found {} gate when expecting XOR or AND in {desc}",
                        circuit.gate(g).op()
                    ),
                );

                context.errors.insert(wire.clone());
            }
        }
    }

    if !got_xor {
        // XOR not found
        follow_err(context, format!("No XOR gate after {desc}"));
    }

    if !got_and {
        // AND not found
        follow_err(context, format!("No AND gate after {desc}"));
    }
}

fn follow_carrya(circuit: &mut Circuit, context: &mut FollowContext, wire: String) {
    context.stack.push(format!("Carry A ({wire})"));

    follow_carry_ab(circuit, context, wire, "carry A");

    context.stack.pop();
}

fn follow_carryb(circuit: &mut Circuit, context: &mut FollowContext, wire: String) {
    context.stack.push(format!("Carry B ({wire})"));

    follow_carry_ab(circuit, context, wire, "carry B");

    context.stack.pop();
}

fn follow_carry_ab(circuit: &mut Circuit, context: &mut FollowContext, wire: String, desc: &str) {
    // Find gates with thsi wire as an input
    let gates = circuit.find_gates_with_inconn(&wire);

    // Should go to OR
    let mut got_or = false;

    for g in gates {
        match circuit.gate(g).op() {
            Op::Or => {
                // Found OR
                follow_carry(circuit, context, circuit.gate(g).out_wire());
                got_or = true;
            }
            _ => {
                // Unexpected gate
                follow_err(
                    context,
                    format!(
                        "Found {} gate when expecting XOR or AND in {desc}",
                        circuit.gate(g).op()
                    ),
                );

                context.errors.insert(wire.clone());
            }
        }
    }

    if !got_or {
        // OR not found
        follow_err(context, format!("No OR gate after {desc}"));
    }
}

#[cfg(debug_assertions)]
fn follow_err(context: &FollowContext, msg: String) {
    println!("ERR: {}: {msg}", context.stack.join(","))
}

#[cfg(not(debug_assertions))]
fn follow_err(_context: &FollowContext, _msg: String) {}

#[cfg(debug_assertions)]
fn check_circuit(circuit: &mut Circuit) -> Vec<usize> {
    // Run the circuit
    circuit.run();

    // Get number of bits in the output
    let xbits = circuit.count_bits('x');
    let ybits = circuit.count_bits('y');

    let incorrect: Vec<usize> = (0..xbits)
        .filter(|&bit| {
            let ok = [
                (0, 0, 0b00, false),
                (1, 0, 0b01, false),
                (0, 1, 0b01, false),
                (1, 1, 0b10, false),
                (0, 0, 0b01, true),
                (1, 0, 0b10, true),
                (0, 1, 0b10, true),
                (1, 1, 0b11, true),
            ]
            .iter()
            .any(|&(xbit, ybit, mut expected, carry)| {
                if !carry || bit != 0 {
                    // Set up inputs
                    let mut wires = FxHashMap::default();

                    for i in 0..xbits {
                        wires.insert(
                            format!("x{:02}", i),
                            (carry && i == bit - 1) || (xbit == 1 && i == bit),
                        );
                    }

                    for i in 0..ybits {
                        wires.insert(
                            format!("y{:02}", i),
                            (carry && i == bit - 1) || (ybit == 1 && i == bit),
                        );
                    }

                    circuit.run_with(wires);

                    let actual = circuit.get_value('z');

                    expected <<= bit;

                    if actual != expected {
                        println!(
                            "Bit {bit} ({xbit} + {ybit} {}) is incorrect",
                            if carry { "with carry" } else { "no carry" }
                        );
                        println!("    actual {actual:064b}");
                        println!("  expected {expected:064b}");
                    }

                    actual != expected
                } else {
                    false
                }
            });

            ok
        })
        .collect();

    incorrect
}

// Input parsing

fn parse_input(input: &str) -> Circuit {
    let mut sections = input.split("\n\n");

    let s1 = sections.next().unwrap();
    let s2 = sections.next().unwrap();

    let mut init_wires = FxHashMap::default();
    let mut inconnections: FxHashMap<String, Vec<(usize, usize)>> = FxHashMap::default();
    let mut outconnections: FxHashMap<String, usize> = FxHashMap::default();

    s1.lines().for_each(|l| {
        let mut split = l.split(':');

        let name = split.next().unwrap().to_string();
        let value = split.next().unwrap().trim_start().parse::<u8>().unwrap();

        init_wires.insert(name, value == 1);
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

            // Add connections
            inconnections.entry(in1).or_default().push((num, 0));
            inconnections.entry(in2).or_default().push((num, 1));

            assert!(outconnections.insert(out_wire.clone(), num).is_none());

            Gate::new(in_wires, out_wire, op)
        })
        .collect();

    Circuit::new(init_wires, gates, inconnections, outconnections)
}

#[cfg(test)]
mod tests;
