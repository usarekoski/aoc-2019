use itertools::Itertools;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Opcode {
    number: i32,
    mode1: i32,
    mode2: i32,
}

impl Opcode {
    fn new(v: i32) -> Opcode {
        Opcode {
            number: v % 100,
            mode1: (v / 100) % 10,
            mode2: (v / 1000) % 10,
        }
    }
}

fn read_param(ins: &Vec<i32>, value: i32, mode: i32) -> i32 {
    if mode == 1 {
        value
    } else {
        ins[value as usize]
    }
}

enum RunOutput {
    Halt,
    Output(i32),
}

struct Interpreter {
    insp: usize,
    ins: Vec<i32>,
    input: Vec<i32>,
}

impl Interpreter {
    fn new(ins: Vec<i32>, input: &[i32]) -> Self {
        Interpreter {
            insp: 0,
            ins: ins,
            input: input.into(),
        }
    }

    fn put_input(&mut self, input: i32) {
        self.input.push(input);
    }

    fn run(&mut self) -> RunOutput {
        loop {
            let insp = self.insp;
            let opcode = Opcode::new(self.ins[insp]);
            match opcode.number {
                1 => {
                    let in1 = read_param(&self.ins, self.ins[insp + 1], opcode.mode1);
                    let in2 = read_param(&self.ins, self.ins[insp + 2], opcode.mode2);
                    let out = self.ins[insp + 3] as usize;
                    self.ins[out] = in1 + in2;
                    self.insp += 4;
                }
                2 => {
                    let in1 = read_param(&self.ins, self.ins[insp + 1], opcode.mode1);
                    let in2 = read_param(&self.ins, self.ins[insp + 2], opcode.mode2);
                    let out = self.ins[insp + 3] as usize;
                    self.ins[out] = in1 * in2;
                    self.insp += 4;
                }
                3 => {
                    let out = self.ins[insp + 1] as usize;
                    self.ins[out] = self.input.remove(0);
                    self.insp += 2;
                }
                4 => {
                    let in1 = read_param(&self.ins, self.ins[insp + 1], opcode.mode1);
                    self.insp += 2;
                    return RunOutput::Output(in1);
                }
                5 => {
                    let in1 = read_param(&self.ins, self.ins[insp + 1], opcode.mode1);
                    let in2 = read_param(&self.ins, self.ins[insp + 2], opcode.mode2);
                    if in1 != 0 {
                        self.insp = in2.try_into().unwrap();
                    } else {
                        self.insp += 3;
                    }
                }
                6 => {
                    let in1 = read_param(&self.ins, self.ins[insp + 1], opcode.mode1);
                    let in2 = read_param(&self.ins, self.ins[insp + 2], opcode.mode2);
                    if in1 == 0 {
                        self.insp = in2.try_into().unwrap();
                    } else {
                        self.insp += 3;
                    }
                }
                7 => {
                    let in1 = read_param(&self.ins, self.ins[insp + 1], opcode.mode1);
                    let in2 = read_param(&self.ins, self.ins[insp + 2], opcode.mode2);
                    let out = self.ins[insp + 3] as usize;
                    self.ins[out] = if in1 < in2 { 1 } else { 0 };
                    self.insp += 4;
                }
                8 => {
                    let in1 = read_param(&self.ins, self.ins[insp + 1], opcode.mode1);
                    let in2 = read_param(&self.ins, self.ins[insp + 2], opcode.mode2);
                    let out: usize = self.ins[insp + 3].try_into().unwrap();
                    self.ins[out] = if in1 == in2 { 1 } else { 0 };
                    self.insp += 4;
                }
                99 => {
                    return RunOutput::Halt;
                }
                _ => panic!("Unexpected opcode {:?}", opcode),
            }
        }
    }
}

pub fn run_amplifiers_1(ins: Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let mut input = 0;

    for i in 0..5 {
        let mut amplifier = Interpreter::new(ins.clone(), &[phase_settings[i], input]);
        if let RunOutput::Output(out) = amplifier.run() {
            input = out;
        } else {
            panic!("Incorrect output!");
        }
    }

    input
}

pub fn run_amplifiers_2(ins: Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let mut amplifiers = vec![];
    let mut input = 0;

    for i in 0..5 {
        let mut amplifier = Interpreter::new(ins.clone(), &[phase_settings[i], input]);
        if let RunOutput::Output(out) = amplifier.run() {
            input = out;
        } else {
            panic!("Incorrect output!");
        }
        amplifiers.push(amplifier);
    }

    'outer: loop {
        for amplifier in amplifiers.iter_mut() {
            amplifier.put_input(input);
            match amplifier.run() {
                RunOutput::Output(out) => {
                    input = out;
                }
                RunOutput::Halt => break 'outer,
            }
        }
    }

    input
}

pub fn solve1(ins: Vec<String>) -> i32 {
    let ins: Vec<i32> = ins[0].split(',').map(|a| a.parse().unwrap()).collect();

    let mut max = 0;
    let mut max_phase_settings = None;
    for phase_settings in (0..=4).permutations(5) {
        let signal = run_amplifiers_1(ins.clone(), phase_settings.clone());
        if signal > max {
            max = signal;
            max_phase_settings = Some(phase_settings);
        }
    }

    dbg!(&max_phase_settings);
    max
}

pub fn solve2(ins: Vec<String>) -> i32 {
    let ins: Vec<i32> = ins[0].split(',').map(|a| a.parse().unwrap()).collect();

    let mut max = 0;
    let mut max_phase_settings = None;
    for phase_settings in (5..=9).permutations(5) {
        let signal = run_amplifiers_2(ins.clone(), phase_settings.clone());
        if signal > max {
            max = signal;
            max_phase_settings = Some(phase_settings);
        }
    }

    dbg!(&max_phase_settings);
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1_1() {
        let ins = vec!["3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string()];
        assert_eq!(solve1(ins), 43210);
    }

    #[test]
    fn test_solve1_2() {
        let ins = vec![
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string(),
        ];
        assert_eq!(solve1(ins), 54321);
    }

    #[test]
    fn test_solve1_3() {
        let ins = vec![
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,\
            1,32,31,31,4,31,99,0,0,0"
                .to_string(),
        ];
        assert_eq!(solve1(ins), 65210);
    }

    #[test]
    fn test_solve2_1() {
        let ins = vec![
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_string(),
        ];
        assert_eq!(solve2(ins), 139629729);
    }

    #[test]
    fn test_solve2_2() {
        let ins = vec![
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,\
            1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,\
            6,99,0,0,0,0,10"
                .to_string(),
        ];
        assert_eq!(solve2(ins), 18216);
    }
}
