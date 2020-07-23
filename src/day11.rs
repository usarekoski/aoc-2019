use std::convert::TryInto;
use std::collections::HashMap;

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

const BLACK: i32 = 0;
const WHITE: i32 = 1;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}
use Direction::*;

const TURN_TO_DIRECTION: [[Direction; 2]; 4] = [
    [Left, Right],
    [Down, Up],
    [Right, Left],
    [Up, Down],
];

const DIRECTION_TO_MOVE: [(i32, i32); 4] = [
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 0),
];

pub fn solve1(ins: Vec<String>) -> i64 {
    let mut pos_to_color: HashMap<(i32, i32), i32> = HashMap::new();
    let mut program = Interpreter::new(ins[0].clone(), &[]);
    let mut position = (0, 0);
    let mut direction = Up;
    loop {
        let color = *pos_to_color.get(&position).unwrap_or(&BLACK);
        program.put_input(color as i64);
        match program.run() {
            RunOutput::Output(out) => {
              pos_to_color.insert(position, out as i32);
            }
            _ => break,
        }

        match program.run() {
            RunOutput::Output(out) => {
                direction = TURN_TO_DIRECTION[direction as usize][out as usize];
                let (dx, dy) = DIRECTION_TO_MOVE[direction as usize];
                position = (position.0 + dx, position.1 + dy);
            }
            _ => break,
        }
    }

    pos_to_color.len() as i64
}

pub fn solve2(ins: Vec<String>) {
    let mut pos_to_color: HashMap<(i32, i32), i32> = HashMap::new();
    let mut program = Interpreter::new(ins[0].clone(), &[]);
    let mut position = (0, 0);
    pos_to_color.insert(position, WHITE);
    let mut direction = Up;
    loop {
        let color = *pos_to_color.get(&position).unwrap_or(&BLACK);
        program.put_input(color as i64);
        match program.run() {
            RunOutput::Output(out) => {
              pos_to_color.insert(position, out as i32);
            }
            _ => break,
        }

        match program.run() {
            RunOutput::Output(out) => {
                direction = TURN_TO_DIRECTION[direction as usize][out as usize];
                let (dx, dy) = DIRECTION_TO_MOVE[direction as usize];
                position = (position.0 + dx, position.1 + dy);
            }
            _ => break,
        }
    }

    for i in (-10..10).rev() {
        for j in -10..50 {
            let color = *pos_to_color.get(&(j, i)).unwrap_or(&BLACK);
            let tile = if color == BLACK { "." } else { "#" };
            print!("{}", tile);
        }
        println!("");
    }
}
