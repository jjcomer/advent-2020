use lazy_static::lazy_static;
use regex::Regex;
use regex::RegexSet;

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|l| l.replace("\n", " ")).collect()
}

lazy_static! {
    static ref BYR_REGEX: Regex = Regex::new(r"byr:(\d{4})\b").unwrap();
    static ref IYR_REGEX: Regex = Regex::new(r"iyr:(\d{4})\b").unwrap();
    static ref EYR_REGEX: Regex = Regex::new(r"eyr:(\d{4})\b").unwrap();
    static ref HGT_REGEX: Regex = Regex::new(r"hgt:(\d{2,3})(cm|in)\b").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"hcl:#[0-9a-f]{6}\b").unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"pid:\d{9}\b").unwrap();
    static ref PASSPORT_CHECK: RegexSet =
        RegexSet::new(&[r"byr:", r"iyr:", r"eyr:", r"hgt:", r"hcl:", r"ecl:", r"pid:",]).unwrap();
}

fn check_passport(passport: &str) -> bool {
    PASSPORT_CHECK.matches(passport).into_iter().count() == PASSPORT_CHECK.len()
}

fn full_check(passport: &str) -> bool {
    if let Some(caps) = BYR_REGEX.captures(passport) {
        let year = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        if !(year >= 1920 && year <= 2002) {
            return false;
        }
    } else {
        return false;
    };
    if let Some(caps) = IYR_REGEX.captures(passport) {
        let year = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        if !(year >= 2010 && year <= 2020) {
            return false;
        }
    } else {
        return false;
    };
    if let Some(caps) = EYR_REGEX.captures(passport) {
        let year = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        if !(year >= 2020 && year <= 2030) {
            return false;
        }
    } else {
        return false;
    };
    if let Some(caps) = HGT_REGEX.captures(passport) {
        let height = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let test = match caps.get(2).unwrap().as_str() {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => return false,
        };
        if !test {
            return false;
        }
    } else {
        return false;
    };

    HCL_REGEX.is_match(passport) && ECL_REGEX.is_match(passport) && PID_REGEX.is_match(passport)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|&passport| check_passport(passport))
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|&passport| full_check(passport))
        .count()
}
