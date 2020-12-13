#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.lines();
    let arrival = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|b| if b == "x" { 0 } else { b.parse().unwrap() })
        .collect();

    (arrival, buses)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(usize, Vec<usize>)) -> usize {
    let (arrival, buses) = input;
    let closest_bus = buses
        .iter()
        .filter(|&b| *b != 0)
        .min_by(|&b1, &b2| (*b1 - (*arrival % *b1)).cmp(&(*b2 - (*arrival % *b2))))
        .unwrap();

    *closest_bus * (*closest_bus - (*arrival % *closest_bus))
}

fn inv_mod(x: i64, p: i64) -> i64 {
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(usize, Vec<usize>)) -> i64 {
    let (_, buses) = input;
    let buses = buses
        .iter()
        .enumerate()
        .filter(|(_, &b)| b != 0)
        .map(|(i, b)| (i as i64, *b as i64))
        .collect::<Vec<(i64, i64)>>();

    let product: i64 = buses.iter().map(|(_, b)| b).product();

    //https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    buses
        .iter()
        .map(|&(a, b)| -a * (product / b) * inv_mod(product / b, b))
        .sum::<i64>()
        .rem_euclid(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "939
        7,13,x,x,59,x,31,19";

        let input = parse_input(&input);
        assert_eq!(295, solve_part1(&input));
    }

    #[test]
    fn part_2() {
        let input = parse_input("123\n17,x,13,19");
        assert_eq!(3417, solve_part2(&input));
    }
}
