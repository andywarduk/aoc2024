use std::error::Error;

use aoc::input::read_input_file;
use fxhash::{FxHashMap, FxHashSet};

mod circuit;
use circuit::{Circuit, Conn, Edge, Gate, Input, Op, Output};

// Convert with: dot -Tsvg -o day24.svg day24.dot

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(24).unwrap();
    let mut circuit = parse_input(&input);

    // Run the circuit
    circuit.run();

    // Count the number of bits in x input
    let bits = circuit.count_bits('x');

    let mut error_list = Vec::new();

    // Loop each bit and fix
    for bit in 0..bits {
        // Follow the bit logic and get any errors
        let (errors, _, _) = follow_bit(&mut circuit, bits, bit);

        // Got any errors?
        if !errors.is_empty() {
            // Yes - should be length 2
            assert_eq!(errors.len(), 2);

            // Swap the wires
            circuit.swap_wire(&errors[0], &errors[1]);

            // Add to error list
            error_list.push(errors);
        }
    }

    // Loop each bit and save layouts
    let mut layout = Vec::new();
    let mut carries = FxHashMap::default();

    for bit in 0..bits {
        // Follow the bit logic and get any errors
        let (errors, gates, carry) = follow_bit(&mut circuit, bits, bit);

        // Should be no errors now
        assert!(errors.is_empty());

        // Save layout
        layout.push(gates);

        if let Some(carry) = carry {
            carries.insert(carry, bit);
        }
    }

    circuit.dump("vis/day24.dot", &layout, &carries, &error_list)?;

    Ok(())
}

#[derive(Debug, Default)]
struct FollowContext {
    bits: usize,
    bit: usize,
    stack: Vec<String>,
    errors: FxHashSet<String>,
    gates: Vec<usize>,
    carry: Option<String>,
}

fn follow_bit(
    circuit: &mut Circuit,
    bits: usize,
    bit: usize,
) -> (Vec<String>, Vec<usize>, Option<String>) {
    // Build context
    let mut context = FollowContext {
        bits,
        bit,
        stack: vec![format!("Bit {bit}")],
        errors: Default::default(),
        gates: Vec::new(),
        carry: None,
    };

    // Follow input wires
    follow_bit_input(circuit, &mut context);

    // Convert error hashset to vector
    (
        context.errors.into_iter().collect::<Vec<_>>(),
        context.gates,
        context.carry,
    )
}

fn follow_bit_input(circuit: &mut Circuit, context: &mut FollowContext) {
    // Build input names
    let inx = Circuit::inoutname('x', context.bit);
    let iny = Circuit::inoutname('y', context.bit);

    // Find gates with x and y inputs for this bit
    let gates = circuit.find_gates_with_inconn2(&inx, &iny);

    // Should be 2 - and XOR and an AND
    let mut xors = 0;
    let mut ands = 0;

    for g in gates {
        match circuit.gate(g).op() {
            Op::Xor => {
                // Found XOR gate
                context.gates.push(g);

                if context.bit == 0 {
                    // Half adder
                    follow_result(context, circuit.gate_outwire(g))
                } else {
                    // Full adder
                    follow_half(circuit, context, circuit.gate_outwire(g))
                }

                xors += 1;
            }
            Op::And => {
                // Got AND gate
                context.gates.push(g);

                if context.bit == 0 {
                    // Half adder
                    follow_carry(circuit, context, circuit.gate_outwire(g))
                } else {
                    // Full adder
                    follow_carryb(circuit, context, circuit.gate_outwire(g))
                }

                ands += 1;
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

    match xors {
        0 => follow_err(context, format!("No XOR gate with {inx} and {iny} found")),
        1 => (),
        _ => follow_err(
            context,
            format!("{xors} XOR gates found with {inx} and {iny}. Expecting 1"),
        ),
    }

    match ands {
        0 => follow_err(context, format!("No AND gate with {inx} and {iny} found")),
        1 => (),
        _ => follow_err(
            context,
            format!("{ands} AND gates found with {inx} and {iny}. Expecting 1"),
        ),
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
        follow_half_or_carry(circuit, context, wire.clone(), "carry", false);
        context.carry = Some(wire);
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
    // Find gates with this wire as an input
    let gates = circuit.find_gates_with_inconn(&wire);

    // Should go to XOR and AND
    let mut xors = 0;
    let mut ands = 0;

    for g in gates {
        match circuit.gate(g).op() {
            Op::Xor => {
                // Found XOR
                if follow {
                    context.gates.push(g);
                    follow_result(context, circuit.gate_outwire(g));
                }

                xors += 1;
            }
            Op::And => {
                // Found AND
                if follow {
                    context.gates.push(g);
                    follow_carrya(circuit, context, circuit.gate_outwire(g));
                }

                ands += 1;
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

    match xors {
        0 => follow_err(context, format!("No XOR gate after {desc}")),
        1 => (),
        _ => follow_err(
            context,
            format!("{xors} XOR gates found after {desc}. Expecting 1"),
        ),
    }

    match ands {
        0 => follow_err(context, format!("No AND gate after {desc}")),
        1 => (),
        _ => follow_err(
            context,
            format!("{ands} AND gates found after {desc}. Expecting 1"),
        ),
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
    let mut ors = 0;

    for g in gates {
        match circuit.gate(g).op() {
            Op::Or => {
                // Found OR
                context.gates.push(g);

                follow_carry(circuit, context, circuit.gate_outwire(g));
                ors += 1;
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

    match ors {
        0 => follow_err(context, format!("No OR gate after {desc}")),
        1 => (),
        _ => follow_err(
            context,
            format!("{ors} OR gates found after {desc}. Expecting 1"),
        ),
    }
}

#[cfg(debug_assertions)]
fn follow_err(context: &FollowContext, msg: String) {
    println!("ERR: {}: {msg}", context.stack.join(" -> "))
}

#[cfg(not(debug_assertions))]
fn follow_err(_context: &FollowContext, _msg: String) {}

// Input parsing

fn parse_input(input: &str) -> Circuit {
    let mut gate_in = FxHashMap::default();
    let mut gate_out = FxHashMap::default();

    let mut split = input.split("\n\n");

    // Build inputs from first split
    let mut inputs = split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut split = l.split(':');

            let name = split.next().unwrap();
            let value = split.next().unwrap().trim_start().parse::<u8>().unwrap();

            Input::new(name, value == 1)
        })
        .collect::<Vec<_>>();

    inputs.sort();

    // Build gates from second split
    let gates = split
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(num, l)| {
            let mut split = l.split_ascii_whitespace();

            let in1 = split.next().unwrap().to_string();
            let op = split.next().unwrap().to_string();
            let in2 = split.next().unwrap().to_string();
            split.next().unwrap();
            let out = split.next().unwrap().to_string();

            gate_in.entry(in1).or_insert_with(Vec::new).push(num);
            gate_in.entry(in2).or_insert_with(Vec::new).push(num);

            gate_out.insert(out, num);

            Gate::new(&op)
        })
        .collect::<Vec<_>>();

    let mut edges = Vec::new();

    // Build gate in edges
    for (wire, to_vec) in &gate_in {
        for &to in to_vec {
            // Where is this connected from?
            if let Some(&from) = gate_out.get(wire) {
                edges.push(Edge::new(wire, Conn::Gate(from), Conn::Gate(to)));
            } else {
                // Must be an input
                let from = inputs
                    .iter()
                    .position(|input| input.name() == wire)
                    .unwrap();

                edges.push(Edge::new(wire, Conn::In(from), Conn::Gate(to)));
            }
        }
    }

    // Build outputs
    let mut outputs = gate_out
        .iter()
        .filter_map(|(wire, _)| {
            if !gate_in.contains_key(wire) {
                // Must be an output
                Some(Output::new(wire))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    outputs.sort();

    // Build gate out edges
    for (i, output) in outputs.iter().enumerate() {
        if let Some(&from) = gate_out.get(output.name()) {
            edges.push(Edge::new(output.name(), Conn::Gate(from), Conn::Out(i)));
        }
    }

    Circuit::new(inputs, outputs, gates, edges)
}
