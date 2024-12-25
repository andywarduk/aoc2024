use std::collections::VecDeque;

use fxhash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct Gate {
    in_wires: [String; 2],
    out_wire: String,
    op: Op,
}

impl Gate {
    pub fn new(in_wires: [String; 2], out_wire: String, op: Op) -> Self {
        Self {
            in_wires,
            out_wire,
            op,
        }
    }

    pub fn out_wire(&self) -> String {
        self.out_wire.clone()
    }

    pub fn op(&self) -> Op {
        self.op.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Circuit {
    init_wires: FxHashMap<String, bool>,
    wires: FxHashMap<String, bool>,
    gates: Vec<Gate>,
    inconnections: FxHashMap<String, Vec<(usize, usize)>>,
    outconnections: FxHashMap<String, usize>,
}

impl Circuit {
    pub fn new(
        init_wires: FxHashMap<String, bool>,
        gates: Vec<Gate>,
        inconnections: FxHashMap<String, Vec<(usize, usize)>>,
        outconnections: FxHashMap<String, usize>,
    ) -> Self {
        Self {
            init_wires,
            wires: Default::default(),
            gates,
            inconnections,
            outconnections,
        }
    }

    pub fn run(&mut self) {
        self.run_with(self.init_wires.clone());
    }

    pub fn run_with(&mut self, wires: FxHashMap<String, bool>) {
        self.wires = wires;

        let mut work = VecDeque::new();

        for (name, &value) in &self.wires {
            work.push_back(Work {
                name: name.clone(),
                value,
            });
        }

        while let Some(work_ent) = work.pop_front() {
            if let Some(conns) = self.inconnections.get(&work_ent.name) {
                for &(gate, inwire) in conns {
                    let gate = &mut self.gates[gate];

                    // Got other wire?
                    let other = &gate.in_wires[1 - inwire];

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

    pub fn gate(&self, g: usize) -> &Gate {
        &self.gates[g]
    }

    pub fn get_value(&self, prefix: char) -> u64 {
        let mut result = 0;
        let mut bit = 0;

        loop {
            let name = format!("{prefix}{bit:02}");

            match self.wires.get(&name) {
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
            let name = format!("{prefix}{bit:02}");

            if !self.wires.contains_key(&name) {
                break;
            }

            bit += 1;
        }

        bit
    }

    pub fn swap_wire(&mut self, w1: &str, w2: &str) {
        let g1 = *self.outconnections.get(w1).unwrap();
        let g2 = *self.outconnections.get(w2).unwrap();

        let mut modify = |w: &str, g| {
            self.outconnections.remove(w);
            self.outconnections.insert(w.to_string(), g);

            if let Some(arr) = self.inconnections.get_mut(w) {
                arr.iter_mut().for_each(|(gate, _)| {
                    if *gate == g {
                        *gate = g
                    }
                })
            }

            self.gates[g].out_wire = w.to_string();
        };

        modify(w1, g2);
        modify(w2, g1);
    }

    pub fn find_gates_with_inconn(&self, wire: &str) -> Vec<usize> {
        self.inconnections
            .get(wire)
            .iter()
            .flat_map(|vec| vec.iter().map(|(g, _)| *g))
            .collect::<Vec<_>>()
    }

    pub fn find_gates_with_inconn2(&self, w1: &str, w2: &str) -> Vec<usize> {
        let gates1 = self
            .inconnections
            .get(w1)
            .iter()
            .flat_map(|vec| vec.iter().map(|(g, _)| *g))
            .collect::<FxHashSet<_>>();

        let gates2 = self
            .inconnections
            .get(w2)
            .iter()
            .flat_map(|vec| vec.iter().map(|(g, _)| *g))
            .collect::<FxHashSet<_>>();

        gates1.intersection(&gates2).copied().collect::<Vec<_>>()
    }

    pub fn inoutname(prefix: char, bit: usize) -> String {
        format!("{prefix}{bit:02}")
    }
}

#[derive(Debug, Clone)]
struct Work {
    name: String,
    value: bool,
}
