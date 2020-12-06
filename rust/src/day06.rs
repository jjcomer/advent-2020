use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|l| l.replace("\n", " ")).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|x| x.chars().collect::<HashSet<char>>())
        .map(|x| {
            if x.contains(&' ') {
                x.len() - 1
            } else {
                x.len()
            }
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|group| {
            let group = group
                .split(" ")
                .map(|doc| doc.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>();

            //rage... turns out taking the intersection of a list of sets is horrible in rust
            let doc_1 = group.first().unwrap();
            doc_1
                .iter()
                .filter(|&x| group[1..].iter().all(|s| s.contains(x)))
                .count()
        })
        .sum()
}
