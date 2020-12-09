use std::collections::VecDeque;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn look_for_sum(list: &VecDeque<i64>, target: i64) -> bool {
    for n in list.iter() {
        let compliment = (target - n).abs();
        if compliment != *n && list.contains(&compliment) {
            return true;
        }
    }
    false
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<i64>) -> usize {
    let mut slider: VecDeque<i64> = input[..25].into_iter().cloned().collect();
    for x in input[25..].iter() {
        if !look_for_sum(&slider, *x) {
            return *x as usize;
        }
        slider.pop_front();
        slider.push_back(*x);
    }
    0
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {
    let part1 = solve_part1(input) as i64;
    for chunk_size in 2..input.len() {
        let mut lower = chunk_size;
        while lower + chunk_size <= input.len() {
            let chunk = &input[lower..lower + chunk_size];
            let sum: i64 = chunk.iter().sum();
            if sum == part1 {
                return chunk.iter().min().unwrap() + chunk.iter().max().unwrap();
            }
            lower += 1;
        }
    }
    0
}
