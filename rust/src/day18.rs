use std::ptr::eq;

use im::Vector;

//https://thealgorists.azurewebsites.net/Algo/Infix

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

fn do_math(op: char, lhs: usize, rhs: usize) -> usize {
    if op == '+' {
        lhs + rhs
    } else {
        lhs * rhs
    }
}

fn eval_p2(input: &Vector<char>) -> usize {
    let mut equation = input.clone();
    let mut operands = Vector::new();
    let mut operators = Vector::new();

    loop {
        // println!(
        //     "EQ:{:?}\nOPER:{:?}\nOPRA:{:?}",
        //     equation, operators, operands
        // );
        if equation.is_empty() {
            loop {
                if let Some(head) = operators.pop_front() {
                    let lhs = operands.pop_front().unwrap();
                    let rhs = operands.pop_front().unwrap();
                    operands.push_front(do_math(head, lhs, rhs));
                } else {
                    break;
                }
            }
            return operands.pop_front().unwrap();
        }
        let head = equation.pop_front().unwrap();
        match head {
            '(' => operators.push_front(head),
            ')' => loop {
                let head = operators.pop_front().unwrap();
                if head == '(' {
                    break;
                } else {
                    let lhs = operands.pop_front().unwrap();
                    let rhs = operands.pop_front().unwrap();
                    operands.push_front(do_math(head, lhs, rhs))
                }
            },
            '+' => operators.push_front(head),
            '*' => loop {
                if *operators.head().unwrap_or(&'*') == '+' {
                    let lhs = operands.pop_front().unwrap();
                    let rhs = operands.pop_front().unwrap();
                    let op = operators.pop_front().unwrap();
                    operands.push_front(do_math(op, lhs, rhs));
                } else {
                    operators.push_front(head);
                    break;
                }
            },
            head => operands.push_front(head.to_digit(10).unwrap() as usize),
        };
    }
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

#[aoc(day18, part2)]
pub fn solve_part2(input: &Vector<Vector<char>>) -> usize {
    input.iter().map(eval_p2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = parse_input("2 * 3 + (4 * 5)");
        assert_eq!(26, solve_part1(&input));
    }

    #[test]
    fn part_2() {
        let tests = vec![
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];
        for (eq, expected) in tests {
            let input = parse_input(eq);
            assert_eq!(expected as usize, eval_p2(input.head().unwrap()));
        }
    }
}
