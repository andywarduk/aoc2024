use std::collections::VecDeque;

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
    gates: Vec<Gate>,
    wirestate: FxHashMap<String, bool>,
    wiretogate: FxHashMap<String, Vec<usize>>,
    gatetoinwire: FxHashMap<usize, Vec<String>>,
    wirefromgate: FxHashMap<String, usize>,
    gatetooutwire: FxHashMap<usize, String>,
}

impl Circuit {
    pub fn new(inputs: Vec<Input>, gates: Vec<Gate>, edges: Vec<Edge>) -> Self {
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
            gates,
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

    #[allow(unused)]
    pub fn run_with(&mut self, inputs: &[Input]) {
        // Set up wire state from passed inputs
        self.wirestate = inputs
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

    pub fn get_value(&self, prefix: char) -> u64 {
        let mut result = 0;
        let mut bit = 0;

        loop {
            let name = Self::inoutname(prefix, bit);

            match self.wirestate.get(&name) {
                Some(&value) => {
                    if value {
                        result |= 2u64.pow(bit as u32);
                    }
                }
                None => break,
            }

            bit += 1;
        }

        result
    }

    pub fn count_bits(&self, prefix: char) -> usize {
        let mut bit = 0;

        loop {
            let name = Self::inoutname(prefix, bit);

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
    }

    pub fn find_gates_with_inconn(&self, wire: &str) -> Vec<usize> {
        self.find_gates_with_inconn_iter(wire).collect()
    }

    pub fn find_gates_with_inconn2(&self, w1: &str, w2: &str) -> Vec<usize> {
        let gates1 = self
            .find_gates_with_inconn_iter(w1)
            .collect::<FxHashSet<_>>();

        let gates2 = self
            .find_gates_with_inconn_iter(w2)
            .collect::<FxHashSet<_>>();

        gates1.intersection(&gates2).copied().collect::<Vec<_>>()
    }

    fn find_gates_with_inconn_iter(&self, wire: &str) -> impl Iterator<Item = usize> + '_ {
        self.wiretogate
            .get(wire)
            .into_iter()
            .flat_map(|vec| vec.iter().copied())
    }

    pub fn inoutname(prefix: char, bit: usize) -> String {
        format!("{prefix}{bit:02}")
    }
}
