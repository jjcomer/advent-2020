#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn count_trees(hill: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let height = hill.len();
    let pattern_width = hill.first().unwrap().len();

    let mut trees = 0;

    for i in 0..height {
        if i * y > height {
            break;
        }
        let location = hill
            .get(i * y)
            .unwrap()
            .get((i * x) % pattern_width)
            .unwrap();
        if *location == '#' {
            trees += 1;
        }
    }

    trees
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> usize {
    count_trees(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> usize {
    count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2)
}
