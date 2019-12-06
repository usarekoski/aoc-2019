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

fn run(mut ins: Vec<i32>, input: i32) -> i32 {
    let mut insp = 0;
    let mut output: Option<i32> = None;
    loop {
        let opcode = Opcode::new(ins[insp]);
        match opcode.number {
            1 => {
                let in1 = read_param(&ins, ins[insp + 1], opcode.mode1);
                let in2 = read_param(&ins, ins[insp + 2], opcode.mode2);
                let out = ins[insp + 3] as usize;
                ins[out] = in1 + in2;
                insp += 4;
            }
            2 => {
                let in1 = read_param(&ins, ins[insp + 1], opcode.mode1);
                let in2 = read_param(&ins, ins[insp + 2], opcode.mode2);
                let out = ins[insp + 3] as usize;
                ins[out] = in1 * in2;
                insp += 4;
            }
            3 => {
                let out = ins[insp + 1] as usize;
                ins[out] = input;
                insp += 2;
            }
            4 => {
                let in1 = read_param(&ins, ins[insp + 1], opcode.mode1);
                output = Some(in1);
                insp += 2;
            }
            5 => {
                let in1 = read_param(&ins, ins[insp + 1], opcode.mode1);
                let in2 = read_param(&ins, ins[insp + 2], opcode.mode2);
                if in1 != 0 {
                    insp = in2.try_into().unwrap();
                } else {
                    insp += 3;
                }
            }
            6 => {
                let in1 = read_param(&ins, ins[insp + 1], opcode.mode1);
                let in2 = read_param(&ins, ins[insp + 2], opcode.mode2);
                if in1 == 0 {
                    insp = in2.try_into().unwrap();
                } else {
                    insp += 3;
                }
            }
            7 => {
                let in1 = read_param(&ins, ins[insp + 1], opcode.mode1);
                let in2 = read_param(&ins, ins[insp + 2], opcode.mode2);
                let out = ins[insp + 3] as usize;
                ins[out] = if in1 < in2 { 1 } else { 0 };
                insp += 4;
            }
            8 => {
                let in1 = read_param(&ins, ins[insp + 1], opcode.mode1);
                let in2 = read_param(&ins, ins[insp + 2], opcode.mode2);
                let out: usize = ins[insp + 3].try_into().unwrap();
                ins[out] = if in1 == in2 { 1 } else { 0 };
                insp += 4;
            }
            99 => {
                break;
            }
            _ => panic!("Unexpected opcode {:?}", opcode),
        }
    }

    output.unwrap()
}

pub fn solve1(ins: Vec<String>) -> i32 {
    let ins: Vec<i32> = ins[0].split(',').map(|a| a.parse().unwrap()).collect();
    let input = 1;

    run(ins, input)
}

pub fn solve2(ins: Vec<String>) -> i32 {
    let ins: Vec<i32> = ins[0].split(',').map(|a| a.parse().unwrap()).collect();
    let input = 5;

    run(ins, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode() {
        assert_eq!(
            Opcode::new(99),
            Opcode {
                number: 99,
                mode1: 0,
                mode2: 0
            }
        );
        assert_eq!(
            Opcode::new(8),
            Opcode {
                number: 8,
                mode1: 0,
                mode2: 0
            }
        );
    }

    #[test]
    fn test2_1() {
        let ins = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(ins.clone(), 8), 1);
        assert_eq!(run(ins.clone(), 7), 0);
    }
}
