use std::convert::TryInto;

const MEMORY_SIZE: usize = 10_000;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Opcode {
    number: i64,
    mode1: OpcodeMode,
    mode2: OpcodeMode,
    mode3: OpcodeMode
}

impl Opcode {
    fn new(v: i64) -> Opcode {
        Opcode {
            number: v % 100,
            mode1: OpcodeMode::new((v / 100) % 10),
            mode2: OpcodeMode::new((v / 1000) % 10),
            mode3: OpcodeMode::new((v / 10000) % 10)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpcodeMode {
    Positional,
    Immediate,
    Relative,
}

impl OpcodeMode {
    fn new(v: i64) -> Self {
        match v {
            0 => OpcodeMode::Positional,
            1 => OpcodeMode::Immediate,
            2 => OpcodeMode::Relative,
            _ => panic!("unexpected opcode mode {:?}", v),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RunOutput {
    Halt,
    Output(i64),
}

struct Interpreter {
    insp: usize,
    ins: Vec<i64>,
    input: Vec<i64>,
    relative_base: i64,
}

impl Interpreter {
    fn new(program: String, input: &[i64]) -> Self {
        let mut ins: Vec<i64> = program.split(',').map(|a| a.parse().unwrap()).collect();
        ins.resize(MEMORY_SIZE, 0);

        Interpreter {
            insp: 0,
            ins: ins,
            input: input.into(),
            relative_base: 0,
        }
    }

    fn put_input(&mut self, input: i64) {
        self.input.push(input);
    }

    fn read_param(&mut self, param: i64, mode: OpcodeMode) -> i64 {
        match mode {
            OpcodeMode::Positional => self.ins[param as usize],
            OpcodeMode::Immediate => param,
            OpcodeMode::Relative => self.ins[(self.relative_base + param) as usize],
        }
    }

    fn write_param(&mut self, param: i64, mode: OpcodeMode, value: i64) {
        match mode {
            OpcodeMode::Positional => {
                self.ins[param as usize] = value;
            },
            OpcodeMode::Immediate => {
                unimplemented!()
            },
            OpcodeMode::Relative => {
                self.ins[(self.relative_base + param) as usize] = value;
            }
        }
    }

    fn run(&mut self) -> RunOutput {
        loop {
            let insp = self.insp;
            let opcode = Opcode::new(self.ins[insp]);
            match opcode.number {
                // Addition
                1 => {
                    let in1 = self.read_param(self.ins[insp + 1], opcode.mode1);
                    let in2 = self.read_param(self.ins[insp + 2], opcode.mode2);
                    let sum = in1 + in2;
                    self.write_param(self.ins[insp + 3], opcode.mode3, sum);
                    self.insp += 4;
                }
                // Product
                2 => {
                    let in1 = self.read_param(self.ins[insp + 1], opcode.mode1);
                    let in2 = self.read_param(self.ins[insp + 2], opcode.mode2);
                    let product = in1 * in2;
                    self.write_param(self.ins[insp + 3], opcode.mode3, product);
                    self.insp += 4;
                }
                // Input
                3 => {
                    let input_value = self.input.remove(0);
                    self.write_param(self.ins[insp + 1], opcode.mode1, input_value);
                    self.insp += 2;
                }
                // Output
                4 => {
                    let in1 = self.read_param(self.ins[insp + 1], opcode.mode1);
                    self.insp += 2;
                    return RunOutput::Output(in1);
                }
                5 => {
                    let in1 = self.read_param(self.ins[insp + 1], opcode.mode1);
                    let in2 = self.read_param(self.ins[insp + 2], opcode.mode2);
                    if in1 != 0 {
                        self.insp = in2.try_into().unwrap();
                    } else {
                        self.insp += 3;
                    }
                }
                6 => {
                    let in1 = self.read_param(self.ins[insp + 1], opcode.mode1);
                    let in2 = self.read_param(self.ins[insp + 2], opcode.mode2);
                    if in1 == 0 {
                        self.insp = in2.try_into().unwrap();
                    } else {
                        self.insp += 3;
                    }
                }
                7 => {
                    let in1 = self.read_param(self.ins[insp + 1], opcode.mode1);
                    let in2 = self.read_param(self.ins[insp + 2], opcode.mode2);
                    let is_greater = if in1 < in2 { 1 } else { 0 };
                    self.write_param(self.ins[insp + 3], opcode.mode3, is_greater);
                    self.insp += 4;
                }
                8 => {
                    let in1 = self.read_param(self.ins[insp + 1], opcode.mode1);
                    let in2 = self.read_param(self.ins[insp + 2], opcode.mode2);
                    let is_equal = if in1 == in2 { 1 } else { 0 };
                    self.write_param(self.ins[insp + 3], opcode.mode3, is_equal);

                    self.insp += 4;
                }
                9 => {
                    let in1 = self.read_param(self.ins[insp + 1], opcode.mode1);
                    self.relative_base += in1;
                    self.insp += 2;
                }
                99 => {
                    return RunOutput::Halt;
                }
                _ => panic!("Unexpected opcode {:?}", opcode),
            }
        }
    }

    fn run_to_halt(&mut self) -> Vec<i64> {
        let mut output = vec![];
        while let RunOutput::Output(out) = self.run() {
            output.push(out);
        }
        output
    }
}

pub fn solve1(ins: Vec<String>) -> Vec<i64> {
    let mut program = Interpreter::new(ins[0].clone(), &[1]);
    program.run_to_halt()
}

pub fn solve2(ins: Vec<String>) -> Vec<i64> {
    let mut program = Interpreter::new(ins[0].clone(), &[2]);
    program.run_to_halt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_itself() {
        let ins = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let ins_parsed: Vec<i64> = ins.split(',').map(|a| a.parse().unwrap()).collect();
        let mut t = Interpreter::new(ins, &[]);
        assert_eq!(t.run_to_halt(), ins_parsed);
    }

    #[test]
    fn test_16_digit_number() {
        let ins = "1102,34915192,34915192,7,4,7,99,0".to_string();
        let mut t = Interpreter::new(ins, &[]);
        assert_eq!(t.run(), RunOutput::Output(1219070632396864));
    }

    #[test]
    fn test_large_number() {
        let ins = "104,1125899906842624,99".to_string();
        let mut t = Interpreter::new(ins, &[]);
        assert_eq!(t.run(), RunOutput::Output(1125899906842624));
    }
}
