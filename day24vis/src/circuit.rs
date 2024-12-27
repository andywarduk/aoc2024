use std::fmt::Write;
use std::fs::write;
use std::{collections::VecDeque, error::Error};

use fxhash::{FxHashMap, FxHashSet};

// Operation

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Op::And => f.write_str("AND"),
            Op::Or => f.write_str("OR"),
            Op::Xor => f.write_str("XOR"),
        }
    }
}

// Gate

#[derive(Debug)]
pub struct Gate {
    op: Op,
}

impl Gate {
    pub fn new(op: &str) -> Self {
        let op = match op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("Invalid op {op}"),
        };

        Self { op }
    }

    pub fn op(&self) -> Op {
        self.op
    }
}

// Connection

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Conn {
    Gate(usize),
    In(usize),
    Out(usize),
}

// Edge

#[derive(Debug)]
pub struct Edge {
    name: String,
    from: Conn,
    to: Conn,
}

impl Edge {
    pub fn new(name: &str, from: Conn, to: Conn) -> Self {
        Self {
            name: name.to_string(),
            from,
            to,
        }
    }
}

// Input

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    name: String,
    state: bool,
}

impl Input {
    pub fn new(name: &str, state: bool) -> Self {
        Self {
            name: name.to_string(),
            state,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

// Output

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Output {
    name: String,
}

impl Output {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

// Circuit

#[derive(Debug)]
pub struct Circuit {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    gates: Vec<Gate>,
    edges: Vec<Edge>,
    wirestate: FxHashMap<String, bool>,
    wiretogate: FxHashMap<String, Vec<usize>>,
    gatetoinwire: FxHashMap<usize, Vec<String>>,
    wirefromgate: FxHashMap<String, usize>,
    gatetooutwire: FxHashMap<usize, String>,
}

impl Circuit {
    pub fn new(
        inputs: Vec<Input>,
        outputs: Vec<Output>,
        gates: Vec<Gate>,
        edges: Vec<Edge>,
    ) -> Self {
        let mut wiretogate = FxHashMap::default();
        let mut gatetoinwire = FxHashMap::default();
        let mut wirefromgate = FxHashMap::default();
        let mut gatetooutwire = FxHashMap::default();

        edges.iter().for_each(|edge| {
            if let Conn::Gate(gate) = edge.to {
                wiretogate
                    .entry(edge.name.clone())
                    .or_insert_with(Vec::new)
                    .push(gate);

                gatetoinwire
                    .entry(gate)
                    .or_insert_with(Vec::new)
                    .push(edge.name.clone());
            }

            if let Conn::Gate(gate) = edge.from {
                wirefromgate.insert(edge.name.clone(), gate);
                gatetooutwire.insert(gate, edge.name.clone());
            }
        });

        Self {
            inputs,
            outputs,
            gates,
            edges,
            wirestate: Default::default(),
            wiretogate,
            gatetoinwire,
            wirefromgate,
            gatetooutwire,
        }
    }

    pub fn run(&mut self) {
        // Set up wire state from inputs
        self.wirestate = self
            .inputs
            .iter()
            .map(|input| (input.name.clone(), input.state))
            .collect();

        self.run_internal();
    }

    fn run_internal(&mut self) {
        // Initialise work queue with input wires
        let mut work = VecDeque::new();

        for name in self.wirestate.keys() {
            work.push_back(name.clone());
        }

        // Process work queue
        while let Some(work_ent) = work.pop_front() {
            // Find gates connected to this wire
            if let Some(conns) = self.wiretogate.get(&work_ent) {
                // Loop gates connected to this wire
                for gn in conns {
                    // Get input values for this gate
                    let invals = self
                        .gatetoinwire
                        .get(gn)
                        .unwrap()
                        .iter()
                        .filter_map(|wire| self.wirestate.get(wire))
                        .collect::<Vec<_>>();

                    if invals.len() == 2 {
                        // Got both wires
                        let gate = &self.gates[*gn];

                        // Perform operation
                        let outval = match gate.op {
                            Op::And => invals[0] & invals[1],
                            Op::Or => invals[0] | invals[1],
                            Op::Xor => invals[0] ^ invals[1],
                        };

                        // Find out wire
                        let outwire = self.gate_outwire(*gn);

                        // Set wire state
                        self.wirestate.insert(outwire.clone(), outval);

                        // Add wire to work queue
                        work.push_back(outwire);
                    }
                }
            }
        }
    }

    pub fn gate(&self, g: usize) -> &Gate {
        &self.gates[g]
    }

    pub fn gate_outwire(&self, gn: usize) -> String {
        self.gatetooutwire.get(&gn).unwrap().to_string()
    }

    pub fn count_bits(&self, prefix: char) -> usize {
        let mut bit = 0;

        loop {
            let name = Circuit::inoutname(prefix, bit);

            if !self.wirestate.contains_key(&name) {
                break;
            }

            bit += 1;
        }

        bit
    }

    pub fn swap_wire(&mut self, w1: &str, w2: &str) {
        // Get gates
        let g1 = *self.wirefromgate.get(w1).unwrap();
        let g2 = *self.wirefromgate.get(w2).unwrap();

        // Function to modify maps
        let mut modify = |w: &str, g| {
            self.wirefromgate.remove(w);
            self.wirefromgate.insert(w.to_string(), g);

            self.gatetooutwire.remove(&g);
            self.gatetooutwire.insert(g, w.to_string());
        };

        // Modify maps
        modify(w1, g2);
        modify(w2, g1);

        // Adjust edges
        self.edges.iter_mut().for_each(|edge| {
            if let Conn::Gate(g) = edge.from {
                if g == g1 {
                    edge.from = Conn::Gate(g2);
                } else if g == g2 {
                    edge.from = Conn::Gate(g1);
                }
            }
        })
    }

    pub fn find_gates_with_inconn(&self, wire: &str) -> Vec<usize> {
        self.wiretogate
            .get(wire)
            .iter()
            .flat_map(|vec| vec.iter().copied())
            .collect::<Vec<_>>()
    }

    pub fn find_gates_with_inconn2(&self, w1: &str, w2: &str) -> Vec<usize> {
        let gates1 = self
            .wiretogate
            .get(w1)
            .iter()
            .flat_map(|vec| vec.iter().copied())
            .collect::<FxHashSet<_>>();

        let gates2 = self
            .wiretogate
            .get(w2)
            .iter()
            .flat_map(|vec| vec.iter().copied())
            .collect::<FxHashSet<_>>();

        gates1.intersection(&gates2).copied().collect::<Vec<_>>()
    }

    pub fn inoutname(prefix: char, bit: usize) -> String {
        format!("{prefix}{bit:02}")
    }

    pub fn dump(
        &self,
        file: &str,
        layout: &[Vec<usize>],
        carries: &FxHashMap<String, usize>,
        error_list: &[Vec<String>],
    ) -> Result<(), Box<dyn Error>> {
        let mut dotfile = String::new();

        dotfile.write_str("digraph {\n")?;
        dotfile.write_str("  fontname=\"Helvetica,Arial,sans-serif\";\n")?;
        dotfile.write_str("  node [fontname=\"Helvetica,Arial,sans-serif\"];\n")?;
        dotfile.write_str("  edge [fontname=\"Helvetica,Arial,sans-serif\"];\n")?;
        dotfile.write_str("  rankdir=\"TB\";\n")?;
        dotfile.write_str("\n")?;

        // Function to return edge style for wire
        let wire_style = |wire: &str| -> &str {
            if *self.wirestate.get(wire).unwrap() {
                "penwidth=2"
            } else {
                "penwidth=1"
            }
        };

        // Add adder cluster for each bit
        for (bit, gates) in layout.iter().enumerate() {
            // Start cluster
            dotfile.write_fmt(format_args!("  subgraph cluster_{bit} {{\n"))?;
            dotfile.write_fmt(format_args!("    label=\"bit {bit}\";\n"))?;
            dotfile.write_str("\n")?;

            // Loop gates in the cluster
            for g in gates {
                let gate = &self.gates[*g];

                // Write node for gate
                dotfile.write_fmt(format_args!(
                    "    g{} [shape=\"box\" style=\"filled\" fillcolor=\"#8888ff\" label=\"{}\"];\n",
                    *g, gate.op
                ))?;

                // Loop all edges
                for edge in &self.edges {
                    // Going to this gate?
                    if matches!(edge.to, Conn::Gate(gn) if gn == *g) {
                        // Is it an input node?
                        if let Conn::In(i) = edge.from {
                            // Add input node
                            dotfile.write_fmt(format_args!(
                                "    i{i} [{} shape=\"circle\" style=\"filled\" fillcolor=\"#88ff88\" label=\"{}\"];\n",
                                wire_style(&self.inputs[i].name),
                                self.inputs[i].name,
                            ))?;
                        }
                    }

                    // Going from this gate?
                    if matches!(edge.from, Conn::Gate(gn) if gn == *g) {
                        // Is it an output node?
                        if let Conn::Out(o) = edge.to {
                            // Add output node
                            dotfile.write_fmt(format_args!(
                                "    o{o} [{} shape=\"circle\" style=\"filled\" fillcolor=\"#ff8888\" label=\"{}\"];\n",
                                wire_style(&self.outputs[o].name),
                                self.outputs[o].name,
                            ))?;
                        }
                    }
                }

                // Does a carry exist for this bit?
                if let Some((name, bit)) = carries.iter().find(|(_, c)| **c == bit) {
                    // Yes - write carry node
                    dotfile.write_fmt(format_args!(
                        "    {} [{} shape=\"circle\" style=\"filled\" fillcolor=\"#ffff88\"];\n",
                        Circuit::inoutname('c', *bit),
                        wire_style(name)
                    ))?;
                }
            }

            // End subgraph
            dotfile.write_str("  }\n")?;
            dotfile.write_str("\n")?;
        }

        // Function to convert connection to node name
        let conn_to_node = |conn| -> String {
            match conn {
                Conn::Gate(n) => format!("g{n}"),
                Conn::In(n) => format!("i{n}"),
                Conn::Out(n) => format!("o{n}"),
            }
        };

        // Carry been added set
        let mut carry_added = FxHashSet::default();

        // Add edges
        for edge in &self.edges {
            // Is this edge a carry?
            if let Some(&bit) = carries.get(&edge.name) {
                // Yes - has the edge from the last gate to carry been added already?
                if !carry_added.contains(&edge.name) {
                    // No - add it
                    dotfile.write_fmt(format_args!(
                        "  {} -> {} [{} headport=\"n\" label=\"{}\"];\n",
                        conn_to_node(edge.from),
                        Circuit::inoutname('c', bit),
                        wire_style(&edge.name),
                        edge.name
                    ))?;

                    // Mark as added
                    carry_added.insert(edge.name.clone());
                }

                // Add edge from carry to destination
                dotfile.write_fmt(format_args!(
                    "  {} -> {} [{} tailport=\"s\" label=\"{}\"];\n",
                    Circuit::inoutname('c', bit),
                    conn_to_node(edge.to),
                    wire_style(&edge.name),
                    edge.name
                ))?;
            } else {
                // Not a carry - just add the edge
                dotfile.write_fmt(format_args!(
                    "  {} -> {} [{} label=\"{}\"];\n",
                    conn_to_node(edge.from),
                    conn_to_node(edge.to),
                    wire_style(&edge.name),
                    edge.name
                ))?;
            }
        }

        // Add error edges
        let mut add_error = |wire, g1, g2| -> Result<(), Box<dyn Error>> {
            // Loop all edges
            for edge in &self.edges {
                // Does this edge go from gate 2?
                if matches!(edge.from, Conn::Gate(gn) if gn == g2) {
                    // Yes - is this a carry wire?
                    let target = if let Some(carry) = carries.get(wire) {
                        // Yes - route to carry
                        Circuit::inoutname('c', *carry)
                    } else {
                        // No - route to gate
                        conn_to_node(edge.to)
                    };

                    // Write the error edge
                    dotfile.write_fmt(format_args!(
                        "  g{g1} -> {} [fontcolor=\"red\" color=\"red\" weight=\"0\" label=\"{}\"];\n",
                        target,
                        wire
                    ))?;
                }
            }

            Ok(())
        };

        for error in error_list {
            // Get gates for error swap
            let gates = [
                *self.wirefromgate.get(&error[0]).unwrap(),
                *self.wirefromgate.get(&error[1]).unwrap(),
            ];

            // Add edges for the error swap
            add_error(&error[1], gates[0], gates[1])?;
            add_error(&error[0], gates[1], gates[0])?;
        }

        // End graph
        dotfile.write_str("}\n")?;

        // Write dot file
        write(file, dotfile)?;

        Ok(())
    }
}
