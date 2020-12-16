use im;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

//departure location: 27-840 or 860-957
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rule {
    name: String,
    min1: i64,
    max1: i64,
    min2: i64,
    max2: i64,
}

lazy_static! {
    static ref RULE: Regex = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

impl Rule {
    fn check_within(&self, n: &i64) -> bool {
        (self.min1 <= *n && self.max1 >= *n) || (self.min2 <= *n && self.max2 >= *n)
    }

    fn new(input: &str) -> Self {
        let captures = RULE.captures(input).unwrap();
        Rule {
            name: captures.get(1).unwrap().as_str().to_string(),
            min1: captures.get(2).unwrap().as_str().parse().unwrap(),
            max1: captures.get(3).unwrap().as_str().parse().unwrap(),
            min2: captures.get(4).unwrap().as_str().parse().unwrap(),
            max2: captures.get(5).unwrap().as_str().parse().unwrap(),
        }
    }
}

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> (Vec<Rule>, Vec<i64>, Vec<Vec<i64>>) {
    let mut sections = input.split("\n\n");
    let raw_rules = sections.next().unwrap().lines();
    let mut raw_your_ticket = sections.next().unwrap().lines().skip(1);
    let raw_other_tickets = sections.next().unwrap().lines().skip(1);

    let rules = raw_rules.map(|r| Rule::new(r)).collect();

    let your_ticket = raw_your_ticket
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let tickets = raw_other_tickets
        .map(|t| t.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, your_ticket, tickets)
}

fn find_bad_values(rules: &Vec<Rule>, tickets: &Vec<Vec<i64>>) -> Vec<i64> {
    tickets
        .iter()
        .flatten()
        .filter(|&n| rules.iter().all(|r| !r.check_within(n)))
        .cloned()
        .collect()
}

fn remove_bad_tickets(rules: &[Rule], tickets: &[Vec<i64>]) -> Vec<Vec<i64>> {
    tickets
        .iter()
        .filter(|&t| !t.iter().any(|n| rules.iter().all(|r| !r.check_within(n))))
        .cloned()
        .collect()
}

fn test_ticket(rules: &[&Rule], ticket: &[i64]) -> bool {
    rules
        .iter()
        .zip(ticket.iter())
        .all(|(r, n)| r.check_within(n))
}

fn check_possibility(
    rules_picked: im::HashSet<Rule>,
    current_rules: im::Vector<Rule>,
    remaining_rules: im::Vector<HashSet<Rule>>,
    tickets: &[Vec<i64>],
) -> Option<Vec<Rule>> {
    if remaining_rules.len() == 0 {
        if tickets.iter().all(|t| {
            current_rules
                .iter()
                .zip(t.iter())
                .all(|(r, n)| r.check_within(n))
        }) {
            return Some(current_rules.iter().cloned().collect());
        } else {
            return None;
        }
    } else {
        for r in remaining_rules
            .front()
            .unwrap()
            .iter()
            .filter(|&r| !rules_picked.contains(r))
        {
            let mut new_rules = current_rules.clone();
            new_rules.push_back(r.clone());
            let mut new_remaining = remaining_rules.clone();
            new_remaining.pop_front();
            if let Some(rules) = check_possibility(
                rules_picked.update(r.clone()),
                new_rules,
                new_remaining,
                tickets,
            ) {
                return Some(rules);
            }
        }
    }
    None
}

fn find_correct_rule(rules: &Vec<Rule>, tickets: &[Vec<i64>]) -> Vec<Rule> {
    let mut rule_options = Vec::new();

    for i in 0..tickets.first().unwrap().len() {
        let possible_rules = rules
            .iter()
            .cloned()
            .filter(|r| tickets.iter().all(|t| r.check_within(t.get(i).unwrap())))
            .collect::<HashSet<Rule>>();
        rule_options.push(possible_rules);
    }

    let correct_rules = check_possibility(
        im::HashSet::new(),
        im::Vector::new(),
        rule_options
            .iter()
            .cloned()
            .collect::<Vec<HashSet<Rule>>>()
            .into(),
        tickets,
    )
    .unwrap();

    correct_rules

    // let possibilities: usize = rule_options.iter().map(|x| x.len()).product();

    // println!("Total Possibilities: {}", possibilities);

    // for rules in rule_options.iter().multi_cartesian_product() {
    //     if rules.len() != rules.iter().collect::<HashSet<_>>().len() {
    //         continue;
    //     }
    //     if tickets
    //         .iter()
    //         .all(|t| rules.iter().zip(t.iter()).all(|(r, n)| r.check_within(n)))
    //     {
    //         println!("{:?}", rules);
    //         return rules.iter().cloned().cloned().cloned().collect();
    //     }
    // }

    // Vec::new()
}

#[aoc(day16, part1)]
pub fn solve_part1((rules, _your_ticket, tickets): &(Vec<Rule>, Vec<i64>, Vec<Vec<i64>>)) -> i64 {
    find_bad_values(rules, tickets).iter().sum()
}

#[aoc(day16, part2)]
pub fn solve_part2((rules, your_ticket, tickets): &(Vec<Rule>, Vec<i64>, Vec<Vec<i64>>)) -> i64 {
    let mut good_tickets = remove_bad_tickets(rules, tickets);
    good_tickets.push(your_ticket.to_owned());
    println!("Good Tickets {}", good_tickets.len());
    let real_rules = find_correct_rule(rules, &good_tickets);

    real_rules
        .iter()
        .zip(your_ticket.iter())
        .filter(|(r, _)| r.name.starts_with("departure"))
        .map(|(_, n)| *n)
        .product()
}
