
fn run(mut ins: Vec<usize>) -> usize {
    let mut insp = 0;
    loop {
        let opcode = ins[insp];
        match opcode {
            1 => {
                let in1 = ins[insp + 1];
                let in2 = ins[insp + 2];
                let out = ins[insp + 3];
                ins[out] = ins[in1] + ins[in2];
            }
            2 => {
                let in1 = ins[insp +1];
                let in2 = ins[insp + 2];
                let out = ins[insp + 3];
                ins[out] = ins[in1] * ins[in2];
            }
            99 => {
                break;
            }
            _ => panic!("Unexpected opcode")
        }
        insp += 4;
    }

    ins[0]
}

fn run_with(mut ins: Vec<usize>, noun: usize, verb: usize) -> usize {
    ins[1] = noun;
    ins[2] = verb;
    run(ins)
}

pub fn solve1(ins: Vec<String>) -> usize {
    let ins: Vec<usize> = ins[0]
        .split(',')
        .map(|a| a.parse().unwrap())
        .collect();

    run_with(ins, 12, 2)
}


pub fn solve2(ins: Vec<String>) -> usize {
    let ins: Vec<usize> = ins[0]
    .split(',')
    .map(|a| a.parse().unwrap())
    .collect();

    // let noun = 0;
    // let mut verb = 0;
    let expected = 19690720;
    for noun in 0..ins.len() {
        for verb in 0..ins.len() {
            if run_with(ins.clone(), noun, verb) == expected {
                return noun*100 + verb;
            }
        }
    }

    panic!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let instructions = vec![1,0,0,0,99];
        assert_eq!(run(instructions), 2);
    }

    #[test]
    fn test1_2() {
        let instructions = vec![2,3,0,3,99];
        assert_eq!(run(instructions), 2);
    }

    #[test]
    fn test1_3() {
        let instructions = vec![2,4,4,5,99,0];
        assert_eq!(run(instructions), 2);
    }

    #[test]
    fn test1_4() {
        let instructions = vec![1,1,1,4,99,5,6,0,99];
        assert_eq!(run(instructions), 30);
    }
}
