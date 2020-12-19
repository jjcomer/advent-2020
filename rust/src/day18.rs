use im::Vector;

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> Vector<Vector<char>> {
    input
        .lines()
        .map(|l| l.replace(" ", "").chars().collect())
        .collect()
}

// (2 + 6) * 2 + 2 + 4
fn get_number(equation: Vector<char>) -> (usize, Vector<char>) {
    // println!("GET NUM {:?}", equation);
    if '(' == *equation.head().unwrap() {
        let end_index = find_matching(equation.clone());
        (
            solve(equation.clone().slice(1..end_index)),
            equation.clone().slice(end_index + 1..),
        )
    } else {
        (
            equation.head().unwrap().to_digit(10).unwrap() as usize,
            equation.clone().slice(1..),
        )
    }
}

fn solve(equation: Vector<char>) -> usize {
    let (next_number, rest_equation) = get_number(equation.clone());

    // println!("SOLVE {} {:?}", next_number, rest_equation);
    if rest_equation.is_empty() {
        return next_number;
    }

    let non_operator = rest_equation.clone().slice(1..);
    let next_number2 = solve(non_operator.clone());
    // println!("SOLVE2 {} {} {:?}", next_number, next_number2, non_operator);
    match *rest_equation.head().unwrap() {
        '*' => next_number * next_number2,
        '+' => next_number + next_number2,
        _ => panic!("Unexpected character {} {:?}", next_number, rest_equation),
    }
}

fn find_matching(equation: Vector<char>) -> usize {
    let mut counter = 0;
    for (i, c) in equation.iter().enumerate() {
        match *c {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => (),
        }
        if counter == 0 {
            // println!("Found matching {}", i);
            return i;
        }
    }
    panic!("Couldn't find the matching paren {:?}", equation);
}

fn add_parens(equation: Vector<char>) -> Vector<char> {
    let mut new_equation = equation.clone();
    new_equation
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Vector<Vector<char>>) -> usize {
    input
        .iter()
        .map(|e| {
            solve(
                e.clone()
                    .iter()
                    .rev()
                    .cloned()
                    .map(|c| match c {
                        '(' => ')',
                        ')' => '(',
                        _ => c,
                    })
                    .collect(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = parse_input("2 * 3 + (4 * 5)");
        assert_eq!(26, solve_part1(&input));
    }
}
