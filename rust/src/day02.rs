//5-6 x: xxxxxmxf
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSE_INPUT: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

pub struct Password {
    min: usize,
    max: usize,
    check: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let line = PARSE_INPUT.captures(l).unwrap();
            Password {
                min: line.get(1).unwrap().as_str().parse().unwrap(),
                max: line.get(2).unwrap().as_str().parse().unwrap(),
                check: line.get(3).unwrap().as_str().chars().next().unwrap(),
                password: line.get(4).unwrap().as_str().to_owned(),
            }
        })
        .collect()
}

fn check_password(password: &Password) -> bool {
    let mut counter = 0;
    for c in password.password.chars() {
        if c == password.check {
            counter += 1;
        }
    }
    counter >= password.min && counter <= password.max
}

fn check_password_2(password: &Password) -> bool {
    let char_a = password.password.as_bytes()[password.min - 1] as char == password.check;
    let char_b = password.password.as_bytes()[password.max - 1] as char == password.check;

    (char_a && !char_b) || (!char_a && char_b)
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Password>) -> usize {
    input.iter().filter(|&p| check_password(p)).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Password>) -> usize {
    input.iter().filter(|&p| check_password_2(p)).count()
}
