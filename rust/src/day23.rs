use im::hashset;
use im::HashMap;
use im::Vector;
use itertools::Itertools;
use num::range;

#[aoc_generator(day23)]
pub fn parse_input(input: &str) -> Vector<usize> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn do_turn(mut cups: Vector<usize>, max: usize) -> Vector<usize> {
    let current = cups.pop_front().unwrap();
    cups.push_back(current);
    let (dropped_cups, mut remaining_cups) = cups.split_at(3);

    let mut destination = 0;
    for x in range(0, current).rev() {
        if x == 0 {
            destination = if dropped_cups.contains(&max) {
                if dropped_cups.contains(&(max - 1)) {
                    if dropped_cups.contains(&(max - 2)) {
                        remaining_cups.index_of(&(max - 3)).unwrap()
                    } else {
                        remaining_cups.index_of(&(max - 2)).unwrap()
                    }
                } else {
                    remaining_cups.index_of(&(max - 1)).unwrap()
                }
            } else {
                remaining_cups.index_of(&max).unwrap()
            };
            break;
        }
        if !dropped_cups.contains(&x) {
            destination = remaining_cups.index_of(&x).unwrap();
            break;
        }
    }

    if destination + 1 == remaining_cups.len() {
        remaining_cups.append(dropped_cups);
        remaining_cups
    } else {
        let (mut front, back) = remaining_cups.split_at(destination + 1);
        front.append(dropped_cups);
        front.append(back);
        front
    }
}

fn do_turn2(mut cups: HashMap<usize, usize>, current: usize) -> (HashMap<usize, usize>, usize) {
    let max = cups.len();
    let one = *cups.get(&current).unwrap();
    let two = *cups.get(&one).unwrap();
    let three = *cups.get(&two).unwrap();
    let removed_cups = hashset![one, two, three];

    let mut destination = if current > 1 { current - 1 } else { max };
    while removed_cups.contains(&destination) {
        destination = if destination > 1 {
            destination - 1
        } else {
            max
        };
    }

    let destination_pointer = *cups.get(&destination).unwrap();
    let next_current = *cups.get(&three).unwrap();
    cups.insert(current, next_current);
    cups.insert(destination, one);
    cups.insert(three, destination_pointer);

    (cups.clone(), next_current)
}

fn final_result(mut cups: Vector<usize>) -> String {
    loop {
        let current = cups.pop_front().unwrap();
        if current == 1 {
            return cups.iter().join("");
        } else {
            cups.push_back(current)
        }
    }
}

fn final_result2(cups: HashMap<usize, usize>) -> usize {
    let one = cups.get(&1).unwrap();
    let two = cups.get(one).unwrap();

    (*one) * (*two)
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &Vector<usize>) -> String {
    let mut cups = input.clone();
    let max = *cups.iter().max().unwrap();
    for _ in 0..100 {
        cups = do_turn(cups, max);
    }
    final_result(cups)
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &Vector<usize>) -> usize {
    let mut cups = HashMap::new();
    for (a, b) in input.iter().tuple_windows() {
        cups.insert(*a, *b);
    }

    for (a, b) in (10..=1_000_000 as usize).tuple_windows() {
        cups.insert(a, b);
    }

    cups.insert(1_000_000, *input.head().unwrap());
    cups.insert(*input.last().unwrap(), 10);

    let mut current = *input.head().unwrap();
    for _ in 0..10_000_000 {
        let result = do_turn2(cups, current);
        cups = result.0;
        current = result.1;
    }
    final_result2(cups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_ish() {
        let input = parse_input("389125467");
        assert_eq!(149245887792, solve_part2(&input));
    }
}
