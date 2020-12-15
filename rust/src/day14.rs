use num::Num;
use std::collections::HashMap;

pub enum Action {
    Mask {
        add: usize,
        remove: usize,
        floating: Vec<(usize, usize)>,
    },
    Set(usize, usize),
}

//mask = 100110001110110011001X101110X1XX10X1

fn parse_mask(mask: &str) -> Action {
    let remove = <usize as Num>::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
    let add = <usize as Num>::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    let mut floating_strings = vec![mask.replace("1", "_").replace("0", "_")];
    let mut floating = Vec::new();

    while floating_strings.len() != 0 {
        let next_string = floating_strings.pop().unwrap();
        if let Some(_i) = next_string.find('X') {
            floating_strings.push(next_string.replacen("X", "0", 1));
            floating_strings.push(next_string.replacen("X", "1", 1));
        } else {
            let add = <usize as Num>::from_str_radix(&next_string.replace("_", "0"), 2).unwrap();
            let remove = <usize as Num>::from_str_radix(&next_string.replace("_", "1"), 2).unwrap();
            floating.push((add, remove));
        }
    }

    Action::Mask {
        add,
        remove,
        floating,
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|l| {
            let mut first_split = l.trim().split(" = ");
            let operator = first_split.next().unwrap();
            let number = first_split.next().unwrap();
            if operator == "mask" {
                parse_mask(number)
            } else {
                let mem = &operator[operator.find("[").unwrap() + 1..operator.len() - 1];
                Action::Set(mem.parse().unwrap(), number.parse().unwrap())
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Vec<Action>) -> usize {
    let mut add_mask = 0;
    let mut remove_mask = 0;
    let mut memory = HashMap::new();

    for action in input {
        match action {
            Action::Mask {
                add,
                remove,
                floating: _floating,
            } => {
                add_mask = *add;
                remove_mask = *remove;
            }
            Action::Set(location, value) => {
                memory.insert(*location, (value & remove_mask) | add_mask);
            }
        }
    }

    memory.values().sum()
}

fn compute_memory(location: usize, add: usize, floating: &[(usize, usize)]) -> Vec<usize> {
    let base_memory = location | add;

    floating
        .iter()
        .map(|&(add, remove)| (base_memory & remove) | add)
        .collect()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Vec<Action>) -> usize {
    let mut add_mask = 0;
    let mut floating_mask = Vec::new();
    let mut memory = HashMap::new();

    for action in input {
        match action {
            Action::Mask {
                add,
                remove: _remove,
                floating,
            } => {
                add_mask = *add;
                floating_mask = floating.to_owned();
            }
            Action::Set(location, value) => {
                for m in compute_memory(*location, add_mask, &floating_mask) {
                    memory.insert(m, *value);
                }
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1() {
    //     let input = parse_input(
    //         "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    //     mem[8] = 11
    //     mem[7] = 101
    //     mem[8] = 0",
    //     );

    //     assert_eq!(165, solve_part1(&input));
    // }

    #[test]
    fn part2() {
        let input = parse_input(
            "mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1",
        );

        assert_eq!(208, solve_part2(&input));
    }
}
