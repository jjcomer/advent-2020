use itertools::iproduct;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    for (x, y) in iproduct!(input, input) {
        if x != y && x + y == 2020 {
            return x * y;
        }
    }
    -1
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    for (x, y, z) in iproduct!(input, input, input) {
        if x != y && y != z && z != x && x + y + z == 2020 {
            return x * y * z;
        }
    }
    -1
}

pub fn filter_by_bit_pos<'a>(
    input: &'a [&'a str],
    pos: usize,
    by: char,
) -> impl Iterator<Item = &'a &'a str> {
    input
        .iter()
        .filter(move |line| line.chars().nth(0).unwrap() == by)
}
