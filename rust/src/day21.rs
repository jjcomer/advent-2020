use im::HashMap;
use im::HashSet;
use itertools::Itertools;
use std::{error::Error, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}
//mxmxvkd kfcds sqjhc nhms (contains dairy, fish)

impl FromStr for Food {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" (contains ");
        let ingredients = split
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let allergens = split
            .next()
            .unwrap()
            .replace(")", "")
            .replace(",", "")
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> HashSet<Food> {
    input.lines().map(|i| i.trim().parse().unwrap()).collect()
}

fn collect_allergens(food: &HashSet<Food>) -> HashSet<String> {
    food.iter()
        .fold(HashSet::new(), |acc, f| acc.union(f.allergens.clone()))
}

fn collect_possible_ingredients(food: &HashSet<Food>, allergen: &str) -> HashSet<String> {
    let mut allergen_foods = food.iter().filter_map(|f| {
        if f.allergens.contains(allergen) {
            Some(f.ingredients.clone())
        } else {
            None
        }
    });
    let first = allergen_foods.next().unwrap();
    allergen_foods.fold(first, |acc, f| acc.intersection(f))
}

#[aoc(day21, part1)]
fn solve_part1(input: &HashSet<Food>) -> usize {
    let known_allergens = collect_allergens(input);
    let mut allergens = known_allergens
        .iter()
        .map(|a| collect_possible_ingredients(input, a));
    let first_allergens = allergens.next().unwrap();
    let all_allergens = allergens.fold(first_allergens, |acc, a| acc.union(a));

    // println!("{:?} {:?}", known_allergens, all_allergens);

    input
        .iter()
        .flat_map(|f| {
            f.ingredients
                .clone()
                .relative_complement(all_allergens.clone())
        })
        // .inspect(|f| {
        //     dbg! {f};
        // })
        .count()
}

#[aoc(day21, part2)]
fn solve_part2(input: &HashSet<Food>) -> String {
    let mut known_allergens = HashMap::new();
    for allergen in collect_allergens(input).iter().sorted() {
        known_allergens.insert(
            allergen.clone(),
            collect_possible_ingredients(input, allergen),
        );
    }

    println!("{:?}", known_allergens.iter().sorted());
    "Solve it manually :)".to_string()
}
//{"fish": {"ttbrlvd"}, "eggs": {"htxsjf"}, "peanuts": {"lmds"}, "shellfish": {"cbmjz"}, "nuts": {"bbbl"}, "dairy": {"cfzdnz"}, "soy": {"cmbcm"}, "wheat": {"dvnbh"}}
// cfzdnz,htxsjf,ttbrlvd,bbbl,lmds,cbmjz,cmbcm,dvnbh
//
// IntoIter([("dairy", {"cbmjz", "cfzdnz", "bbbl", "lmds"}), ("eggs", {"htxsjf", "ttbrlvd", "dvnbh"}), ("fish", {"bbbl", "ttbrlvd", "cbmjz"}), ("nuts", {"bbbl", "cmbcm"}), ("peanuts", {"bbbl", "lmds"}), ("shellfish", {"bbbl", "cbmjz"}), ("soy", {"cmbcm"}), ("wheat", {"cmbcm", "dvnbh"})])

#[cfg(test)]
mod tests {
    use super::*;
    use apply::Apply;

    #[test]
    fn part1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)"
            .apply(parse_input);

        assert_eq!(5, solve_part1(&input));
    }
}
