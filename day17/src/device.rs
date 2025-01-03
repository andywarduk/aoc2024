type RegType = u64;

#[derive(Debug)]
pub struct Device<'a> {
    reg: [RegType; 3],
    pc: usize,
    program: Option<&'a [u8]>,
    out: Vec<u8>,
    debug: bool,
}

impl<'a> Device<'a> {
    pub fn new() -> Self {
        Self {
            reg: [0; 3],
            pc: 0,
            program: None,
            out: Vec::new(),
            debug: false,
        }
    }

    pub fn reg(mut self, reg: Reg, val: RegType) -> Self {
        self.reg[reg as usize] = val;
        self
    }

    pub fn program(mut self, program: &'a [u8]) -> Self {
        self.program = Some(program);
        self
    }

    #[allow(dead_code)]
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    #[allow(dead_code)]
    pub fn get_reg(&self, reg: Reg) -> RegType {
        self.reg[reg as usize]
    }

    pub fn get_output(&self) -> &Vec<u8> {
        &self.out
    }

    pub fn run(&mut self) -> bool {
        if self.debug {
            self.print_regs();
        }

        if let Some(program) = self.program {
            while self.pc < program.len() {
                let op = program[self.pc];
                let operand_val = program[self.pc + 1];

                if self.debug {
                    print!("pc:{:02} [{},{}]: ", self.pc, op, operand_val);
                };

                self.pc += 2;

                match op {
                    0 => {
                        // adv
                        let operand = Operand::from_u8(operand_val);

                        if self.debug {
                            print!(
                                "adv - a /= 2^{} ({})",
                                operand.debug(),
                                Self::debug_val((2 as RegType).pow(operand.value(self) as u32))
                            );
                        };

                        self.reg[Reg::A as usize] /= (2 as RegType).pow(operand.value(self) as u32);
                    }
                    1 => {
                        // bxl
                        if self.debug {
                            print!("bxl - b ^= {}", Self::debug_val(operand_val as RegType));
                        };

                        self.reg[Reg::B as usize] ^= operand_val as RegType;
                    }
                    2 => {
                        // bst
                        let operand = Operand::from_u8(operand_val);

                        if self.debug {
                            print!("bst - b = {} % 8", operand.debug());
                        };

                        self.reg[Reg::B as usize] = operand.value(self) % 8;
                    }
                    3 => {
                        // jnz
                        let operand = Operand::from_u8(operand_val);

                        if self.debug {
                            print!("jnz a");
                        };

                        if self.reg[Reg::A as usize] != 0 {
                            if self.debug {
                                print!(" - jump to {}", operand.debug());
                            }

                            self.pc = operand.value(self) as usize;
                        } else if self.debug {
                            print!(" - no jump");
                        }
                    }
                    4 => {
                        // bxc
                        if self.debug {
                            print!("bxc - b ^= c");
                        };

                        self.reg[Reg::B as usize] ^= self.reg[Reg::C as usize];
                    }
                    5 => {
                        // out
                        let operand = Operand::from_u8(operand_val);

                        if self.debug {
                            print!(
                                "out {} % 8 ({})",
                                operand.debug(),
                                Self::debug_val(operand.value(self) % 8)
                            );
                        };

                        self.out.push((operand.value(self) % 8) as u8);
                    }
                    6 => {
                        // bdv
                        let operand = Operand::from_u8(operand_val);

                        if self.debug {
                            print!(
                                "bdv - b = a / 2^{} ({})",
                                operand.debug(),
                                Self::debug_val((2 as RegType).pow(operand.value(self) as u32))
                            );
                        };

                        self.reg[Reg::B as usize] = self.reg[Reg::A as usize]
                            / (2 as RegType).pow(operand.value(self) as u32);
                    }
                    7 => {
                        // cdv
                        let operand = Operand::from_u8(operand_val);

                        if self.debug {
                            print!(
                                "cdv - c = a / 2^{} ({})",
                                operand.debug(),
                                Self::debug_val((2 as RegType).pow(operand.value(self) as u32))
                            );
                        };

                        self.reg[2] = self.reg[Reg::A as usize]
                            / (2 as RegType).pow(operand.value(self) as u32);
                    }
                    _ => panic!("Invalid opcode {op}"),
                }

                if self.debug {
                    self.print_regs();
                }
            }
        }

        if self.debug {
            println!("end, output={:?}", self.out);
        }

        true
    }

    fn print_regs(&self) {
        println!(
            "\na={} b={} c={}",
            Self::debug_val(self.reg[0]),
            Self::debug_val(self.reg[1]),
            Self::debug_val(self.reg[2]),
        );
    }

    fn debug_val(v: RegType) -> String {
        format!("[{v} x{v:x} o{v:o} b{v:b}]")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    A = 0,
    B = 1,
    C = 2,
}

impl Reg {
    fn debug(&self) -> &str {
        match self {
            Reg::A => "a",
            Reg::B => "b",
            Reg::C => "c",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Lit(u8),
    Reg(Reg),
}

impl Operand {
    fn from_u8(val: u8) -> Self {
        if (0..=3).contains(&val) {
            Operand::Lit(val)
        } else {
            let reg = match val {
                4 => Reg::A,
                5 => Reg::B,
                6 => Reg::C,
                _ => panic!("unrecognised operand"),
            };
            Operand::Reg(reg)
        }
    }

    fn value(&self, device: &Device) -> RegType {
        match self {
            Operand::Lit(val) => *val as RegType,
            Operand::Reg(reg) => device.reg[(*reg) as usize],
        }
    }

    fn debug(&self) -> String {
        match self {
            Operand::Lit(lit) => Device::debug_val(*lit as RegType),
            Operand::Reg(reg) => reg.debug().to_string(),
        }
    }
}
