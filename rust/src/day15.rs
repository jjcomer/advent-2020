use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|d| d.parse().unwrap())
        .collect()
}

fn run_game(starting_nums: &Vec<i64>, max_turn: i64) -> i64 {
    let mut memory = HashMap::new();
    let mut last_number = *starting_nums.last().unwrap();

    for (i, num) in starting_nums.iter().enumerate() {
        memory.insert(*num, i as i64 + 1);
    }

    for turn in starting_nums.len() as i64 + 1..=max_turn {
        if memory.contains_key(&last_number) {
            let old_last = last_number;
            last_number = (turn - 1) - memory.get(&last_number).unwrap();
            memory.insert(old_last, turn - 1);
        } else {
            memory.insert(last_number, turn - 1);
            last_number = 0;
        }
    }

    last_number
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Vec<i64>) -> i64 {
    run_game(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {
    run_game(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let tests = vec![(436, "0,3,6")];

        for (answer, input) in tests {
            let input = parse_input(input);
            assert_eq!(answer, solve_part1(&input));
        }
    }
}
