use im::vector;
use im::HashMap;
use im::Vector;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    Constant(char),
    Options(Vector<Vector<usize>>),
}

pub type Grammar = HashMap<usize, Rule>;

// 28: 94 108
// 40: 117 117 | 33 117
// 34: 117 46 | 33 53
// 33: "a"
fn parse_rule(input: &str) -> (usize, Rule) {
    let mut split = input.split(": ");
    let rule_num = split.next().unwrap().parse().unwrap();
    let raw_rule = split.next().unwrap();

    let rule = if raw_rule.starts_with("\"") {
        Rule::Constant(raw_rule.chars().nth(1).unwrap())
    } else {
        let options = raw_rule
            .split(" | ")
            .map(|r| r.split(" ").map(|n| n.parse().unwrap()).collect())
            .collect();
        Rule::Options(options)
    };

    (rule_num, rule)
}

#[aoc_generator(day19)]
pub fn parse_input(input: &str) -> (Vector<Vector<char>>, Grammar) {
    let mut split = input.split("\n\n");
    let rules = split
        .next()
        .unwrap()
        .lines()
        .map(|l| parse_rule(l.trim()))
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        });
    let tests = split
        .next()
        .unwrap()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();

    (tests, rules)
}

fn parse_string(
    test: Vector<char>,
    grammar: &Grammar,
    rule: usize,
) -> Vector<Option<Vector<char>>> {
    match grammar.get(&rule).unwrap() {
        Rule::Constant(c) if test.head() == Some(c) => vector![Some(test.clone().slice(1..))],
        Rule::Constant(_) => vector![None],
        Rule::Options(options) => {
            let results = options
                .iter()
                .flat_map(|r| {
                    r.iter().fold(vector![Some(test.clone())], |m, r| {
                        m.iter()
                            .flat_map(|t| match t {
                                Some(m) if !m.is_empty() => parse_string(m.clone(), grammar, *r),
                                _ => vector![None],
                            })
                            .collect()
                    })
                })
                .filter(|x| x.is_some())
                .collect::<Vector<Option<Vector<char>>>>();

            if results.is_empty() {
                return vector![None];
            } else if results.contains(&Some(vector![])) {
                return vector![Some(vector![])];
            } else {
                return results;
            }
        }
    }
}

#[aoc(day19, part1)]
pub fn solve_part1((tests, grammar): &(Vector<Vector<char>>, Grammar)) -> usize {
    tests
        .iter()
        .filter(|&t| parse_string(t.clone(), grammar, 0).contains(&Some(vector![])))
        .count()
}

#[aoc(day19, part2)]
pub fn solve_part2((tests, grammar): &(Vector<Vector<char>>, Grammar)) -> usize {
    let mut grammar = grammar.clone();
    grammar.insert(8, Rule::Options(vector![vector![42], vector![42, 8]]));
    grammar.insert(
        11,
        Rule::Options(vector![vector![42, 31], vector![42, 11, 31]]),
    );

    tests
        .iter()
        .filter(|&t| parse_string(t.clone(), &grammar, 0).contains(&Some(vector![])))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        let input = parse_input(
            r#"42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: "a"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: "b"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1

        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#,
        );

        assert_eq!(12, solve_part2(&input));
    }
}
