use apply::Apply;
use im::HashSet;
use im::Vector;

#[aoc_generator(day22)]
pub fn parse_input(input: &str) -> (Vector<usize>, Vector<usize>) {
    let mut hands = input.split("\n\n").map(|player| {
        player
            .lines()
            .skip(1)
            .map(|c| c.trim().parse().unwrap())
            .collect()
    });

    (hands.next().unwrap(), hands.next().unwrap())
}

fn play_game(mut p1: Vector<usize>, mut p2: Vector<usize>) -> Vector<usize> {
    while !p1.is_empty() && !p2.is_empty() {
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        if p1_card > p2_card {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }
    }
    if p1.is_empty() {
        p2.clone()
    } else {
        p1.clone()
    }
}

fn play_game2(mut p1: Vector<usize>, mut p2: Vector<usize>) -> (bool, Vector<usize>) {
    let mut seen_hands = HashSet::new();
    seen_hands.insert((p1.clone(), p2.clone()));
    while !p1.is_empty() && !p2.is_empty() {
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        if p1_card <= p1.len() && p2_card <= p2.len() {
            let (winner, _) = play_game2(p1.take(p1_card), p2.take(p2_card));
            if winner {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            } else {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
        } else {
            if p1_card > p2_card {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            } else {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
        }
        if seen_hands.contains(&(p1.clone(), p2.clone())) {
            return (true, p1.clone());
        } else {
            seen_hands.insert((p1.clone(), p2.clone()));
        }
    }
    if p1.is_empty() {
        (false, p2.clone())
    } else {
        (true, p1.clone())
    }
}

fn score_game(hand: Vector<usize>) -> usize {
    hand.iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x * (i + 1))
        .sum()
}

#[aoc(day22, part1)]
pub fn solve_part1((p1, p2): &(Vector<usize>, Vector<usize>)) -> usize {
    play_game(p1.clone(), p2.clone()).apply(score_game)
}

#[aoc(day22, part2)]
pub fn solve_part2((p1, p2): &(Vector<usize>, Vector<usize>)) -> usize {
    let (_, hand) = play_game2(p1.clone(), p2.clone());
    score_game(hand)
}
