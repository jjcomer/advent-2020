use itertools::Itertools;

use cached::proc_macro::cached;
use cached::stores::UnboundCache;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    let (_, diff1, diff3) = input
        .iter()
        .sorted()
        .fold((0, 0, 1), |(last, diff1, diff3), next| match next - last {
            1 => (*next, diff1 + 1, diff3),
            2 => (*next, diff1, diff3),
            3 => (*next, diff1, diff3 + 1),
            _ => panic!("Unknown diff {} {} {}", next, last, next - last),
        });
    diff1 * diff3
}

#[cached(
    convert = r#"{format!("{}:{}",goal,current)}"#,
    create = "{UnboundCache::new()}",
    type = "UnboundCache<String,usize>"
)]
fn find_combo(sockets: &[usize], current: usize, goal: usize) -> usize {
    if current + 3 == goal {
        return 1;
    }

    let next_max = current + 3;
    let mut path_count = 0;
    for path in sockets.iter().take_while(|&&x| x <= next_max) {
        let new_sockets = sockets
            .iter()
            .filter(|&&x| x != *path)
            .cloned()
            .collect::<Vec<usize>>();
        path_count += find_combo(&new_sockets, *path, goal);
    }

    path_count
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    let sockets: Vec<usize> = input.iter().sorted().cloned().collect();
    let goal = sockets.iter().max().unwrap() + 3;

    find_combo(&sockets, 0, goal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_a() {
        let input = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";

        let input = parse_input(input);
        assert_eq!(8, solve_part2(&input));
    }

    #[test]
    fn part2_b() {
        let input = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";

        let input = parse_input(input);
        assert_eq!(19208, solve_part2(&input));
    }
}
