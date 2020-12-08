use std::collections::HashSet;

pub enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut inst = l.split(" ");
            match inst.next().unwrap() {
                "nop" => Instruction::NOP(inst.next().unwrap().parse().unwrap()),
                "acc" => Instruction::ACC(inst.next().unwrap().parse().unwrap()),
                "jmp" => Instruction::JMP(inst.next().unwrap().parse().unwrap()),
                _ => panic!("UNKNOWN CMD {}", l),
            }
        })
        .collect()
}

fn check_for_loop(input: &Vec<Instruction>, swap: i32) -> (bool, i32) {
    let mut seen_instructions = HashSet::new();
    let mut prog_counter: i32 = 0;
    let mut acc = 0;

    loop {
        if seen_instructions.contains(&prog_counter) {
            return (true, acc);
        }
        if prog_counter == input.len() as i32 {
            return (false, acc);
        }

        seen_instructions.insert(prog_counter);

        match input.get(prog_counter as usize).unwrap() {
            Instruction::NOP(inc) => {
                if swap == prog_counter {
                    prog_counter += inc;
                } else {
                    prog_counter += 1;
                }
            }
            Instruction::ACC(inc) => {
                prog_counter += 1;
                acc += inc;
            }
            Instruction::JMP(inc) => {
                if swap == prog_counter {
                    prog_counter += 1;
                } else {
                    prog_counter += inc;
                }
            }
        }
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> i32 {
    let (_, result) = check_for_loop(input, -1);
    result
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> i32 {
    for (i, inst) in input.iter().enumerate() {
        let (was_loop, result) = match inst {
            Instruction::ACC(_) => continue,
            _ => check_for_loop(input, i as i32),
        };
        if !was_loop {
            return result;
        }
    }
    -1
}
