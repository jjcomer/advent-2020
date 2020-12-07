use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

//light orange bags contain 1 dark maroon bag, 3 dim maroon bags, 5 striped green bags, 2 pale aqua bags.
lazy_static! {
    static ref RULE: Regex = Regex::new(r"^(\w+ \w+) bags? contain (.*).$").unwrap();
    static ref CONTAINING_RULE: Regex = Regex::new(r"(\d+) (.+) bags?\.?").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bag {
    colour: String,
    contents: Vec<(String, usize)>,
}

impl Bag {
    fn new(input: &str) -> Self {
        let mut first_cut = input.split(" bags contain ");
        let colour = first_cut.next().unwrap().to_owned();
        let contains = first_cut
            .next()
            .unwrap()
            .split(", ")
            .filter_map(|b| {
                if b == "no other bags." {
                    return None;
                }
                let rule = CONTAINING_RULE.captures(b).unwrap();
                Some((
                    rule.get(2).unwrap().as_str().to_owned(),
                    rule.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                ))
            })
            .collect();

        Bag {
            colour,
            contents: contains,
        }
    }
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> HashMap<String, Bag> {
    input
        .lines()
        .map(|s| s.trim())
        .map(|l| Bag::new(l))
        .fold(HashMap::new(), |mut acc, b| {
            acc.insert(b.colour.to_owned(), b.clone());
            acc
        })
}

fn find_gold_bag(bags: &HashMap<String, Bag>, bag: &str) -> bool {
    let bag = bags.get(bag).unwrap();
    for (colour, _) in bag.contents.iter() {
        if colour == "shiny gold" {
            return true;
        } else {
            if find_gold_bag(bags, colour) {
                return true;
            }
        }
    }
    false
}

fn count_containing_bags(bags: &HashMap<String, Bag>, bag: &str) -> usize {
    let bag = bags.get(bag).unwrap();
    let mut sum = 0;

    for (colour, number) in bag.contents.iter() {
        let added_num = count_containing_bags(bags, colour) + 1;
        sum += number * added_num;
    }

    sum
}

#[aoc(day7, part1)]
pub fn solve_part1(bags: &HashMap<String, Bag>) -> usize {
    let mut count = 0;
    for colour in bags.keys() {
        if find_gold_bag(&bags, colour) {
            count += 1;
        }
    }
    count
}

#[aoc(day7, part2)]
pub fn solve_part2(bags: &HashMap<String, Bag>) -> usize {
    count_containing_bags(bags, "shiny gold")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        let input = "light orange bags contain 1 dark maroon bag, 3 dim maroon bags, 5 striped green bags, 2 pale aqua bags.";
        let expected = Bag {
            colour: "light orange".to_string(),
            contents: vec![
                ("dark maroon".to_string(), 1),
                ("dim maroon".to_string(), 3),
                ("striped green".to_string(), 5),
                ("pale aqua".to_string(), 2),
            ],
        };

        assert_eq!(expected, Bag::new(input));
    }

    #[test]
    fn part_2() {
        let input = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.";
        let input = parse_input(input);

        assert_eq!(126, solve_part2(&input));
    }

    #[test]
    fn part_2_b() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";
        let input = parse_input(input);

        assert_eq!(32, solve_part2(&input));
    }
}
